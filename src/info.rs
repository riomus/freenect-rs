use ffi::*;

pub fn supported_subdevices () ->  Vec<FreenectDeviceFlags> {
    unsafe {
        let result = freenect_supported_subdevices () as u8;

        let mut vec = Vec::new ();
        switch_freenect_device_flags! (result;
            {vec.push (FreenectDeviceFlags::MOTOR);},
            {vec.push (FreenectDeviceFlags::CAMERA);},
            {vec.push (FreenectDeviceFlags::AUDIO);});

        vec
    }
}

pub fn get_video_mode_count () -> usize {
    unsafe {
        freenect_get_video_mode_count () as usize
    }
}
