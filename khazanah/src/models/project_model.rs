use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::path::Path;

use conlang::project;
use conlang::Project;
use gtk::glib;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;

    use conlang::Project;

    #[derive(Default)]
    pub struct ProjectModel {
        pub project: RefCell<Option<Project>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ProjectModel {
        const NAME: &'static str = "KhzProjectModel";
        type Type = super::ProjectModel;
    }

    impl ObjectImpl for ProjectModel {}
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
    }

    /// Gets a reference to the current project.
    pub fn project(&self) -> Ref<Option<Project>> {
        self.imp().project.borrow()
    }

    /// Gets a mutable reference to the current project.
    pub fn project_mut(&self) -> RefMut<Option<Project>> {
        self.imp().project.borrow_mut()
    }

    /// Returns true when a project is loaded.
    pub fn is_loaded(&self) -> bool {
        self.imp().project.borrow().is_some()
    }

    /// Load project from a file.
    pub fn load_file<P: AsRef<Path>>(&self, path: P) -> Result<(), project::Error> {
        let project = Project::load_file(path)?;
        self.set_project(Some(project));
        Ok(())
    }

    /// Save project to a file.
    pub fn save_file<P: AsRef<Path>>(&self, path: P) -> Result<(), project::Error> {
        match self.project_mut().as_mut() {
            Some(project) => {
                project.save_file(path)?;
                Ok(())
            }
            None => Err(project::Error::WrongMimeType),
        }
    }

    /// Update the state of the project.
    pub fn update<F>(&self, f: F)
    where
        F: Fn(&mut Project),
    {
        if let Some(project) = self.project_mut().as_mut() {
            f(project);
        }
    }
}

impl Default for ProjectModel {
    fn default() -> Self {
        Self::new()
    }
}

// impl glib::PropertySetNested for ProjectModel {
//     fn set_nested<F: FnOnce(&mut Self::SetNestedValue)>(&self, f: F) {

//     }
// }
