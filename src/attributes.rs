use ffi::*;
use utils::const_c_to_string;

pub struct DeviceAttributes {
    pub serial : String,
}

impl DeviceAttributes {
    pub unsafe fn from_linked_list (list : *mut FreenectDeviceAttributes, n : usize) -> Vec<DeviceAttributes> {
        let mut atts = Vec::with_capacity (n);

        let mut iter = list;
        while !iter.is_null () {
            let e = DeviceAttributes {
                serial : const_c_to_string ((*iter).camera_serial),
            };
            atts.push (e);

            let to_free = iter;
            iter = (*iter).next;

            freenect_free_device_attributes (to_free);
        }

        atts
    }

    pub fn get_serial (&self) -> &str {
        &self.serial
    }
}
