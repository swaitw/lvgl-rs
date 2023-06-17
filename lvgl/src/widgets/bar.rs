use crate::support::AnimationState;
use crate::widgets::Bar;
use crate::{LvResult, NativeObject};

impl Bar {
    /// Set minimum and the maximum values of the bar
    //pub fn set_range(&mut self, min: i16, max: i16) -> LvResult<()> {
    //    unsafe {
    //        lvgl_sys::lv_bar_set_range(self.core.raw()?.as_mut(), min, max);
    //    }
    //    Ok(())
    //}

    /// Set a new value on the bar
    pub fn set_value(&mut self, value: i32, anim: AnimationState) -> LvResult<()> {
        unsafe {
            lvgl_sys::lv_bar_set_value(self.core.raw()?.as_mut(), value, anim.into());
        }
        Ok(())
    }

    /// Gets the current value of the bar
    pub fn get_value(&self) -> LvResult<i32> {
        unsafe { Ok(lvgl_sys::lv_bar_get_value(self.core.raw()?.as_ptr())) }
    }
}
/*
/// The different parts, of a bar object.
pub enum BarPart {
    /// The background of the bar.
    Background,
    /// The indicator of the bar.
    /// This is what moves/changes, depending on the bar's value.
    Indicator,
}

impl From<BarPart> for u8 {
    fn from(component: BarPart) -> Self {
        match component {
            BarPart::Background => lvgl_sys::LV_BAR_PART_BG as u8,
            BarPart::Indicator => lvgl_sys::LV_BAR_PART_INDIC as u8,
        }
    }
}
*/
