use crate::ae_sys::AEGP_ItemH;
use crate::aegp::*;
use crate::*;
use after_effects_sys::{AEGP_ProjectH, AEGP_TimeDisplay3};

define_suite!(
    /// These functions access and modify project data. Support for multiple projects is included to prepare for future expansion; After Effects currently adheres to the single project model.
    /// To save project-specific data in After Effects’ preferences (and thus, outside the projects themselves), use the Persistent Data Suite.
    /// Use caution: the functions for opening and creating projects do not save changes to the project currently open when they are called!
    /// Notes from bindings author: These functions do not work during project setup - do not use them in GlobalSetup.
    ProjectSuite,
    AEGP_ProjSuite6,
    kAEGPProjSuite,
    kAEGPProjSuiteVersion6
);

impl ProjectSuite {
    pub fn new() -> Result<Self, Error> { crate::Suite::new() }

    /// Currently will never return more than 1. After Effects can have only one project open at a time.
    pub fn get_num_projects(&self) -> Result<i32, Error> {
        Ok(call_suite_fn_single!(self, AEGP_GetNumProjects -> ae_sys::A_long)?.into())
    }

    /// Retrieves a specific project by index. as per `num_projects`, this will only ever take 0 as an argument.
    pub fn get_project_by_index(&self, proj_index: i32) -> Result<ProjectHandle, Error> {
        Ok(ProjectHandle::from_raw(
            call_suite_fn_single!( self, AEGP_GetProjectByIndex -> AEGP_ProjectH, proj_index)?,
        ))
    }

     /// Retrieves the current time display settings.
    pub fn get_project_time_display_config(&self, proj_handle: ProjectHandle) -> Result<TimeDisplayConfig, Error> {
        Ok(call_suite_fn_single!( self, AEGP_GetProjectTimeDisplay -> AEGP_TimeDisplay3, proj_handle.into())?.into())
    }


     /// Specified [sic] the settings to be used for displaying time.
    pub fn set_project_time_display_config(&self, proj_handle: ProjectHandle, config: TimeDisplayConfig) -> Result<(), Error> {
        let config: AEGP_TimeDisplay3 = config.into();
        call_suite_fn!( self, AEGP_SetProjectTimeDisplay, proj_handle.into(), &config)?;
        Ok(())
    }


    /// Obtain the current project name
    pub fn get_project_name(&self, proj_handle: ProjectHandle) -> Result<String, Error> {

        let mut buffer = [0i8; (ae_sys::AEGP_MAX_PROJ_NAME_SIZE + 1) as _];
        call_suite_fn!(self, AEGP_GetProjectName, proj_handle.into(), buffer.as_mut_ptr() as *mut _)?;
        Ok(unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()) }.to_string_lossy().into_owned())
    }

    /// Get the path of the project (empty string the project hasn’t been saved yet). The path is a handle to a NULL-terminated A_UTF16Char string,
    pub fn get_project_path(&self, proj_handle: ProjectHandle) -> Result<String, Error> {
        let mem_handle = call_suite_fn_single!(self, AEGP_GetProjectPath -> ae_sys::AEGP_MemHandle, proj_handle.into())?;

        Ok(unsafe {
            U16CString::from_ptr_str(MemHandle::<u16>::from_raw(mem_handle)?.lock()?.as_ptr()).to_string_lossy()
        })
    }
    /// Returns TRUE if the project has been modified since it was opened.
    pub fn project_is_dirty(&self, proj_handle: ProjectHandle) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, AEGP_ProjectIsDirty -> ae_sys::Boolean, proj_handle.into())? != 0)
    }

    /// Saves the entire project to the specified full path.
    pub fn save_project_to_path(&self, proj_handle: ProjectHandle, path: &str) -> Result<(), Error> {
        let path = widestring::U16CString::from_str(path).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(self, AEGP_SaveProjectToPath, proj_handle.into(), path.as_ptr())?;
        Ok(())
    }

    /// Saves the project to the specified path. 
    /// NOTE: This will overwrite an existing file.
    pub fn save_project_as(&self, proj_handle: ProjectHandle, path: &str) -> Result<(), Error> {
        let path = widestring::U16CString::from_str(path).map_err(|_| Error::InvalidParms)?;
        call_suite_fn!(self, AEGP_SaveProjectAs, proj_handle.into(), path.as_ptr())?;
        Ok(())
    }

    /// Get the root of the project, which After Effects also treats as a folder.
    pub fn get_project_root_folder(&self, proj_handle: ProjectHandle) -> Result<ItemHandle, Error> {
        Ok(ItemHandle::from_raw(
            call_suite_fn_single!(self, AEGP_GetProjectRootFolder -> AEGP_ItemH, proj_handle.into())?,
        ))
    }

    /// Opens a project from the supplied path, and returns its AEGP_ProjectH.
    /// The file path is a NULL-terminated UTF-16 string with platform separators.
    /// NOTE: Will close the current project without saving it first!
    pub fn open_project_from_path(&self, path: &str) -> Result<ProjectHandle, Error> {
        let path = widestring::U16CString::from_str(path).map_err(|_| Error::InvalidParms)?;
        Ok(ProjectHandle::from_raw(call_suite_fn_single!(self, AEGP_OpenProjectFromPath -> AEGP_ProjectH, path.as_ptr())?))
    }

    /// Creates a new project. NOTE: Will close the current project without saving it first!
    pub fn new_project(&self) -> Result<ProjectHandle, Error> {
        Ok(ProjectHandle::from_raw(call_suite_fn_single!(self, AEGP_NewProject -> AEGP_ProjectH)?))
    }

    /// Retrieves the project bit depth.
    pub fn get_project_bit_depth(
        &self,
        proj_handle: ProjectHandle,
    ) -> Result<ProjectBitDepth, Error> {
        Ok(call_suite_fn_single!( self, AEGP_GetProjectBitDepth -> ae_sys::A_char, proj_handle.into())?.into())
    }

    // Set the project bit depth, undoable.
    pub fn set_project_bit_depth(
        &self,
        proj_handle: ProjectHandle,
        bit_depth: ProjectBitDepth,
    ) -> Result<(), Error> {
        Ok(call_suite_fn!(self, AEGP_SetProjectBitDepth, proj_handle.into(), bit_depth.into())?.into())
    }

}

register_handle!(AEGP_ProjectH);
define_handle_wrapper!(ProjectHandle, AEGP_ProjectH);

define_enum! {
    ae_sys::AEGP_ProjBitDepth,
    ProjectBitDepth {
        BitDepthU8 = ae_sys::AEGP_ProjBitDepth_8,
        BitDepthU16 = ae_sys::AEGP_ProjBitDepth_16,
        BitDepthF32 = ae_sys::AEGP_ProjBitDepth_32,
    }
}

define_enum! {
    ae_sys::AEGP_SourceTimecodeDisplayMode,
    SourceTimecodeDisplayMode {
        SourceTimeCode = ae_sys::AEGP_SourceTimecode_SOURCE_TIMECODE,
        Zero = ae_sys::AEGP_SourceTimecode_ZERO,
    }
}

define_enum! {
    ae_sys::AEGP_TimeDisplayMode,
    TimeDisplayMode {
        TimeCode = ae_sys::AEGP_TimeDisplay_TIMECODE,
        Frames = ae_sys::AEGP_TimeDisplay_FRAMES,
    }
}

define_enum! {
    ae_sys::AEGP_FramesDisplayMode,
    FrameDisplayMode {
        ZeroBased = ae_sys::AEGP_Frames_ZERO_BASED,
        OneBased = ae_sys::AEGP_Frames_ONE_BASED,
        TimeCodeConversion = ae_sys::AEGP_Frames_TIMECODE_CONVERSION,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TimeDisplayConfig {
    pub source_timecode_display_mode: SourceTimecodeDisplayMode,
    pub footage_display_mode: TimeDisplayMode,
    pub display_drop_frames: bool,
    pub use_feet_frames: bool,
    pub time_base: u8,
    pub frames_per_foot: u8,
    pub frame_display_mode: FrameDisplayMode,
}

impl Into<ae_sys::AEGP_TimeDisplay3> for TimeDisplayConfig {
    fn into(self) -> ae_sys::AEGP_TimeDisplay3 {
        ae_sys::AEGP_TimeDisplay3 {
            display_mode: self.source_timecode_display_mode.into(),
            footage_display_mode: self.footage_display_mode.into(),
            display_dropframeB: self.display_drop_frames.into(),
            use_feet_framesB: self.use_feet_frames.into(),
            timebaseC: self.time_base as ae_sys::A_char,
            frames_per_footC: self.frames_per_foot as ae_sys::A_char,
            frames_display_mode: self.frame_display_mode.into(),
        }
    }
}

impl From<ae_sys::AEGP_TimeDisplay3> for TimeDisplayConfig {
    fn from(time_display: ae_sys::AEGP_TimeDisplay3) -> Self {
        Self {
            source_timecode_display_mode: time_display.display_mode.into(),
            footage_display_mode: time_display.footage_display_mode.into(),
            display_drop_frames: time_display.display_dropframeB != 0,
            use_feet_frames: time_display.use_feet_framesB != 0,
            time_base: time_display.timebaseC as u8,
            frames_per_foot: time_display.frames_per_footC as u8,
            frame_display_mode: time_display.frames_display_mode.into(),
        }
    }
}