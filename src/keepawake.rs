use windows::{
    core::Error as WindowsError,
    Win32::System::Power::{
        SetThreadExecutionState, 
        ES_DISPLAY_REQUIRED, 
        ES_SYSTEM_REQUIRED,
        EXECUTION_STATE,
        ES_CONTINUOUS
    }
};

#[derive(Clone)]
pub struct Options {
    pub display: bool,
    pub idle: bool
}

#[derive(Clone)]
pub struct KeepAwake {
    options: Option<Options>,
    previous: EXECUTION_STATE
}

impl Drop for KeepAwake {
    fn drop(&mut self) {
        unsafe {
            SetThreadExecutionState(self.previous);
        }
    }
}

impl KeepAwake {
    pub fn new(options: Option<Options>) -> Result<Self, WindowsError> {
        let keepawake = KeepAwake {
            options,
            previous: Default::default()
        };

        Ok(keepawake)
    }

    pub fn set_options(&mut self, options: Options) {
        self.options = Some(options);
    }

    pub fn activate(&mut self) -> Result<(), WindowsError> {
        let mut esflags = ES_CONTINUOUS;

        if self.options.as_mut().unwrap().display {
            esflags |= ES_DISPLAY_REQUIRED;
        }

        if self.options.as_mut().unwrap().idle {
            esflags |= ES_SYSTEM_REQUIRED;
        }

        unsafe {
            self.previous = SetThreadExecutionState(esflags);
            if self.previous == EXECUTION_STATE(0) {
                return Err(WindowsError::from_win32());
            }

            Ok(())
        }
    }
}