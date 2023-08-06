use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::path::Path;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use khazanah_core::project;
use khazanah_core::Project;

mod imp {
    use std::cell::Cell;

    use super::*;

    use gtk::glib::subclass::{Signal, SignalType};
    use once_cell::sync::Lazy;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::ProjectModel)]
    pub struct ProjectModel {
        #[property(name = "opened", type = bool, get = Self::get_opened)]
        #[property(name = "title", type = Option<String>, get = Self::get_title)]
        pub project: RefCell<Option<Project>>,

        #[property(get, set)]
        pub dirty: Cell<bool>,

        #[property(get, set)]
        pub path: RefCell<Option<String>>,

        pub title: RefCell<String>,
    }

    impl ProjectModel {
        fn get_title(&self) -> Option<String> {
            self.project
                .borrow()
                .as_ref()
                .map(|project| {
                    let s = &project.language().meta().name;
                    if s.is_empty() {
                        "New Project".to_string()
                    } else {
                        s.to_owned()
                    }
                })
                .map(|s| {
                    if self.dirty.get() {
                        "*".to_string() + &s
                    } else {
                        s
                    }
                })
        }

        fn get_opened(&self) -> bool {
            self.project.borrow().is_some()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ProjectModel {
        const NAME: &'static str = "KhzProjectModel";
        type Type = super::ProjectModel;
    }

    impl ObjectImpl for ProjectModel {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("project-updated")
                    .param_types(Vec::<SignalType>::new())
                    .build()]
            });
            SIGNALS.as_ref()
        }
    }
}

glib::wrapper! {
    /// Wrapper for `Project` structure
    pub struct ProjectModel(ObjectSubclass<imp::ProjectModel>);
}

impl ProjectModel {
    /// Creates a new `ProjectModel`
    pub fn new() -> Self {
        glib::Object::new()
    }

    /// Creates a model from an existing project.
    pub fn from_project(project: Project) -> Self {
        let ret = Self::new();
        ret.set_project(Some(project));
        ret
    }

    /// Sets the current project.
    pub fn set_project(&self, project: Option<Project>) {
        self.imp().project.replace(project);
        self.set_dirty(false);
        self.notify_title();
        self.notify_opened();
        self.emit_by_name::<()>("project-updated", &[]);
    }

    /// Gets a reference to the current project.
    pub fn project(&self) -> Ref<Option<Project>> {
        self.imp().project.borrow()
    }

    /// Gets a mutable reference to the current project.
    pub fn project_mut(&self) -> RefMut<Option<Project>> {
        self.imp().project.borrow_mut()
    }

    pub fn new_project(&self) {
        self.set_project(Some(Project::new()));
        self.imp().path.replace(None);
        self.notify_path();
    }

    /// Loads project from a file.
    pub fn load_file<P: AsRef<Path>>(&self, path: P) -> Result<(), project::ArchiveError> {
        let project = Project::load_file(&path)?;
        self.set_project(Some(project));
        self.set_path(path.as_ref().to_string_lossy().to_string());
        self.set_dirty(false);
        self.notify_title();
        self.notify_opened();
        Ok(())
    }

    /// Saves the project to a file.
    pub fn save_file<P: AsRef<Path>>(&self, path: P) -> Result<(), project::ArchiveError> {
        let result = match self.project_mut().as_mut() {
            Some(project) => {
                project.save_file(&path)?;
                Ok(())
            }
            None => Err(project::ArchiveError::WrongMimeType),
        };
        if result.is_ok() {
            self.set_dirty(false);
            self.set_path(path.as_ref().to_string_lossy().to_string());
            self.notify_title();
        }
        result
    }

    /// Updates the state of the project. Marks the project as dirty.
    pub fn update<F, O>(&self, f: F) -> Option<O>
    where
        F: Fn(&mut Project) -> O,
    {
        let ret = self.project_mut().as_mut().map(f);
        self.set_dirty(true);
        self.notify_title();
        ret
    }

    /// Queries the state of the project.
    pub fn query<F, O>(&self, f: F) -> Option<O>
    where
        F: Fn(&Project) -> O,
    {
        self.project().as_ref().map(f)
    }

    pub fn notify_changes(&self) {
        self.set_dirty(true);
        self.notify_title();
    }
}

impl Default for ProjectModel {
    fn default() -> Self {
        Self::new()
    }
}
