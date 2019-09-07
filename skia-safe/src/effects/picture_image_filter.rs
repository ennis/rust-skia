use crate::prelude::*;
use crate::{ImageFilter, Picture, Rect};
use skia_bindings as sb;
use skia_bindings::{SkImageFilter, SkPicture};

impl RCHandle<SkImageFilter> {
    pub fn from_picture<'a>(
        picture: Picture,
        crop_rect: impl Into<Option<&'a Rect>>,
    ) -> Option<Self> {
        from_picture(picture, crop_rect)
    }
}

impl RCHandle<SkPicture> {
    pub fn as_image_filter<'a>(
        &self,
        crop_rect: impl Into<Option<&'a Rect>>,
    ) -> Option<ImageFilter> {
        self.clone().into_image_filter(crop_rect)
    }

    pub fn into_image_filter<'a>(
        self,
        crop_rect: impl Into<Option<&'a Rect>>,
    ) -> Option<ImageFilter> {
        from_picture(self, crop_rect)
    }
}

pub fn from_picture<'a>(
    picture: Picture,
    crop_rect: impl Into<Option<&'a Rect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkPictureImageFilter_Make(picture.into_ptr(), crop_rect.into().native_ptr_or_null())
    })
}
