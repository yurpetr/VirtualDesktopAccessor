use super::{
    interfaces::{ComIn, IVirtualDesktop, IVirtualDesktopManagerInternal},
    *,
};
use std::fmt::Debug;
use windows::{
    core::{GUID, HSTRING},
    Win32::Foundation::HWND,
};

use super::raw::*;

/*
#[derive(Copy, Clone, PartialEq)]
pub struct Desktop(GUID);

impl Debug for Desktop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Desktop({:?})", self.0)
    }
}

impl Desktop {
    pub(crate) fn empty() -> Desktop {
        Desktop(GUID::default())
    }

    pub fn get_id(&self) -> GUID {
        self.0
    }

    pub fn set_name(&self, name: &str) -> Result<()> {
        com_sta();
        let provider = get_iservice_provider()?;
        let manager = get_ivirtual_desktop_manager_internal(&provider)?;
        let idesktop = get_idesktop_by_guid(&manager, &self.get_id())?;
        set_idesktop_name(&manager, &idesktop, name)
    }

    pub fn get_index(&self) -> Result<u32> {
        com_sta();
        let provider = get_iservice_provider()?;
        let manager = get_ivirtual_desktop_manager_internal(&provider)?;
        let idesktop = get_idesktop_by_guid(&manager, &self.get_id())?;
        let index = get_idesktop_number(&manager, &idesktop)?;
        Ok(index)
    }

    pub fn get_wallpaper(&self) -> Result<String> {
        com_sta();
        let provider = get_iservice_provider()?;
        let manager = get_ivirtual_desktop_manager_internal(&provider)?;
        let idesktop = get_idesktop_by_guid(&manager, &self.get_id())?;
        get_idesktop_wallpaper(&idesktop)
    }

    pub fn set_wallpaper(&self, path: &str) -> Result<()> {
        com_sta();
        let provider = get_iservice_provider()?;
        let manager = get_ivirtual_desktop_manager_internal(&provider)?;
        let idesktop = get_idesktop_by_guid(&manager, &self.get_id())?;
        set_idesktop_wallpaper(&manager, &idesktop, path)
    }
} */

#[derive(Copy, Clone, PartialEq, Debug)]
enum DesktopInternal {
    Index(u32),
    Guid(GUID),
    IndexGuid(u32, GUID),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct DesktopTest(DesktopInternal);

// Impl from u32 to DesktopTest
impl From<u32> for DesktopTest {
    fn from(index: u32) -> Self {
        DesktopTest(DesktopInternal::Index(index))
    }
}

// Impl from i32 to DesktopTest
impl From<i32> for DesktopTest {
    fn from(index: i32) -> Self {
        DesktopTest(DesktopInternal::Index(index as u32))
    }
}

// Impl from GUID to DesktopTest
impl From<GUID> for DesktopTest {
    fn from(guid: GUID) -> Self {
        DesktopTest(DesktopInternal::Guid(guid))
    }
}

// Impl from &GUID to DesktopTest
impl From<&GUID> for DesktopTest {
    fn from(guid: &GUID) -> Self {
        DesktopTest(DesktopInternal::Guid(*guid))
    }
}
impl DesktopTest {
    fn get_ivirtual_desktop(
        &self,
        manager: &IVirtualDesktopManagerInternal,
    ) -> Result<IVirtualDesktop> {
        match &self.0 {
            DesktopInternal::Index(index) => get_idesktop_by_index(manager, *index),
            DesktopInternal::Guid(guid) => get_idesktop_by_guid(manager, guid),
            DesktopInternal::IndexGuid(_, guid) => get_idesktop_by_guid(manager, guid),
        }
    }

    fn internal_get_id(&self, manager: &IVirtualDesktopManagerInternal) -> Result<GUID> {
        match &self.0 {
            DesktopInternal::Index(index) => {
                com_sta();
                let idesktop = get_idesktop_by_index(manager, *index)?;
                get_idesktop_guid(&idesktop)
            }
            DesktopInternal::Guid(guid) => Ok(*guid),
            DesktopInternal::IndexGuid(_, guid) => Ok(*guid),
        }
    }

    /// Get the GUID of the desktop
    pub fn get_id(&self) -> Result<GUID> {
        match &self.0 {
            DesktopInternal::Index(index) => {
                com_sta();
                let provider = get_iservice_provider()?;
                let manager = get_ivirtual_desktop_manager_internal(&provider)?;
                let idesktop = get_idesktop_by_index(&manager, *index)?;
                get_idesktop_guid(&idesktop)
            }
            DesktopInternal::Guid(guid) => Ok(*guid),
            DesktopInternal::IndexGuid(_, guid) => Ok(*guid),
        }
    }

    pub fn get_index(&self) -> Result<u32> {
        match &self.0 {
            DesktopInternal::Index(index) => Ok(*index),
            DesktopInternal::Guid(guid) => {
                com_sta();
                let provider = get_iservice_provider()?;
                let manager = get_ivirtual_desktop_manager_internal(&provider)?;
                let idesktop = get_idesktop_by_guid(&manager, guid)?;
                get_idesktop_index(&manager, &idesktop)
            }
            DesktopInternal::IndexGuid(index, _) => Ok(*index),
        }
    }

    /// Get desktop name
    pub fn get_name(&self) -> Result<String> {
        com_sta();
        let provider = get_iservice_provider()?;
        let manager = get_ivirtual_desktop_manager_internal(&provider)?;
        let idesk = self.get_ivirtual_desktop(&manager);
        get_idesktop_name(&idesk?)
    }

    /// Set desktop name
    pub fn set_name(&self, name: &str) -> Result<()> {
        com_sta();
        let provider = get_iservice_provider()?;
        let manager = get_ivirtual_desktop_manager_internal(&provider)?;
        let idesk = self.get_ivirtual_desktop(&manager);
        set_idesktop_name(&manager, &idesk?, name)
    }

    /// Get desktop wallpaper path
    pub fn get_wallpaper(&self) -> Result<String> {
        com_sta();
        let provider = get_iservice_provider()?;
        let manager = get_ivirtual_desktop_manager_internal(&provider)?;
        let idesk = self.get_ivirtual_desktop(&manager);
        get_idesktop_wallpaper(&idesk?)
    }

    /// Set desktop wallpaper path
    pub fn set_wallpaper(&self, path: &str) -> Result<()> {
        com_sta();
        let provider = get_iservice_provider()?;
        let manager = get_ivirtual_desktop_manager_internal(&provider)?;
        let idesk = self.get_ivirtual_desktop(&manager);
        set_idesktop_wallpaper(&manager, &idesk?, path)
    }
}

/// Get desktop by index or GUID
pub fn get_desktop<T>(desktop: T) -> DesktopTest
where
    T: Into<DesktopTest>,
{
    desktop.into()
}

/// Switch desktop by index or GUID
pub fn switch_desktop<T>(desktop: T) -> Result<()>
where
    T: Into<DesktopTest>,
{
    com_sta();
    let provider = get_iservice_provider()?;
    let manager = get_ivirtual_desktop_manager_internal(&provider)?;
    let idesktop = desktop.into().get_ivirtual_desktop(&manager)?;
    switch_to_idesktop(&manager, &idesktop)
}

/// Remove desktop by index or GUID
pub fn remove_desktop<T, F>(desktop: T, fallback_desktop: F) -> Result<()>
where
    T: Into<DesktopTest>,
    F: Into<DesktopTest>,
{
    com_sta();
    let provider = get_iservice_provider()?;
    let manager = get_ivirtual_desktop_manager_internal(&provider)?;
    let idesktop = desktop.into().get_ivirtual_desktop(&manager)?;
    let fallback_idesktop = fallback_desktop.into().get_ivirtual_desktop(&manager)?;
    remove_idesktop(&manager, &idesktop, &fallback_idesktop)
}

/// Is window on desktop by index or GUID
pub fn is_window_on_desktop<T>(desktop: T, hwnd: HWND) -> Result<bool>
where
    T: Into<DesktopTest>,
{
    com_sta();
    let provider = get_iservice_provider()?;
    let manager_internal = get_ivirtual_desktop_manager_internal(&provider)?;
    let manager = get_ivirtual_desktop_manager(&provider)?;

    // Get desktop of the window
    let desktop_win = get_idesktop_by_window(&manager_internal, &manager, hwnd)?;
    let desktop_win_id = get_idesktop_guid(&desktop_win)?;

    // If ID matches with given desktop, return true
    Ok(desktop_win_id == desktop.into().internal_get_id(&manager_internal)?)
}

/// Move window to desktop by index or GUID
pub fn move_window_to_desktop<T>(desktop: T, hwnd: HWND) -> Result<()>
where
    T: Into<DesktopTest>,
{
    com_sta();
    let provider = get_iservice_provider()?;
    let manager = get_ivirtual_desktop_manager_internal(&provider)?;
    let vc = get_iapplication_view_collection(&provider)?;
    let view = get_iapplication_view_for_hwnd(&vc, hwnd)?;
    let idesktop = desktop.into().get_ivirtual_desktop(&manager)?;
    move_view_to_desktop(&manager, &view, &idesktop)
}

/// Create desktop
pub fn create_desktop() -> Result<DesktopTest> {
    com_sta();
    let provider = get_iservice_provider()?;
    let manager = get_ivirtual_desktop_manager_internal(&provider)?;
    let desktop = create_idesktop(&manager)?;
    let id = get_idesktop_guid(&desktop)?;
    Ok(DesktopTest(DesktopInternal::Guid(id)))
}

/// Get current desktop
pub fn get_current_desktop() -> Result<DesktopTest> {
    com_sta();
    let provider = get_iservice_provider()?;
    let manager = get_ivirtual_desktop_manager_internal(&provider)?;
    let desktop = get_current_idesktop(&manager)?;
    let id = get_idesktop_guid(&desktop)?;
    Ok(DesktopTest(DesktopInternal::Guid(id)))
}

/// Get all desktops
pub fn get_desktops() -> Result<Vec<DesktopTest>> {
    com_sta();
    let provider = get_iservice_provider()?;
    let manager = get_ivirtual_desktop_manager_internal(&provider)?;
    get_idesktops(&manager)?
        .into_iter()
        .enumerate()
        .map(|(i, d)| -> Result<DesktopTest> {
            let mut guid = GUID::default();
            unsafe { d.get_id(&mut guid).as_result()? };
            Ok(DesktopTest(DesktopInternal::IndexGuid(i as u32, guid)))
        })
        .collect()
}

/// Get desktop by window
pub fn get_desktop_by_window(hwnd: HWND) -> Result<DesktopTest> {
    com_sta();
    let provider = get_iservice_provider()?;
    let manager_internal = get_ivirtual_desktop_manager_internal(&provider)?;
    let manager = get_ivirtual_desktop_manager(&provider)?;
    let desktop = get_idesktop_by_window(&manager_internal, &manager, hwnd)?;
    let id = get_idesktop_guid(&desktop)?;
    Ok(DesktopTest(DesktopInternal::Guid(id)))
}

/// Get desktop count
pub fn get_desktop_count() -> Result<u32> {
    com_sta();
    let provider = get_iservice_provider()?;
    let manager = get_ivirtual_desktop_manager_internal(&provider)?;
    let desktops = get_idesktops_array(&manager)?;
    unsafe { desktops.GetCount().map_err(map_win_err) }
}
