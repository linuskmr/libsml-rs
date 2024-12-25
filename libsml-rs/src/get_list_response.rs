//! See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_get_list_response.h>

use std::marker::PhantomData;

use crate::{OctetString, Time};

/// See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_get_list_response.h>
pub struct GetListResponse<'a>(
    pub(crate) *mut libsml_sys::sml_get_list_response,
    PhantomData<&'a ()>,
);

impl<'a> GetListResponse<'a> {
    pub fn new() -> Self {
        GetListResponse(
            unsafe { libsml_sys::sml_get_list_response_init() },
            PhantomData,
        )
    }

    pub fn client_id(&self) -> Option<OctetString> {
        let client_id = unsafe { (*self.0).client_id };
        if client_id.is_null() {
            None
        } else {
            Some(OctetString(client_id, PhantomData))
        }
    }

    pub fn set_client_id(&mut self, client_id: Option<&'a OctetString>) {
        unsafe { (*self.0).client_id = client_id.map_or(std::ptr::null_mut(), |v| v.0) }
    }

    pub fn server_id(&self) -> OctetString {
        OctetString(unsafe { (*self.0).server_id }, PhantomData)
    }

    pub fn set_server_id(&mut self, server_id: &'a OctetString) {
        unsafe { (*self.0).server_id = server_id.0 }
    }

    pub fn list_name(&self) -> Option<OctetString> {
        let list_name = unsafe { (*self.0).list_name };
        if list_name.is_null() {
            None
        } else {
            Some(OctetString(list_name, PhantomData))
        }
    }

    pub fn set_list_name(&mut self, list_name: Option<&'a OctetString>) {
        unsafe { (*self.0).list_name = list_name.map_or(std::ptr::null_mut(), |v| v.0) }
    }

    pub fn act_sensor_time(&self) -> Time<'a> {
        Time(unsafe { (*self.0).act_sensor_time }, PhantomData)
    }

    pub fn set_act_sensor_time(&mut self, act_sensor_time: &'a Time) {
        unsafe { (*self.0).act_sensor_time = act_sensor_time.0 }
    }

    /* pub fn val_list(&self) -> List {
        let val_list_len = unsafe { (*self.0).val_list_len } as usize;
        let mut val_list = Vec::with_capacity(val_list_len);
        for i in 0..val_list_len {
            let val_list = unsafe { *(*self.0).val_list.add(i) };
            val_list.push(ValList(val_list, PhantomData));
        }
        val_list
    } */

    /* pub fn list_signature(&self) -> Option<Signature> {
        Signature(unsafe { (*self.0).list_signature }, PhantomData)
    } */

    pub fn act_gateway_time(&self) -> Option<Time<'a>> {
        let act_gateway_time = unsafe { (*self.0).act_gateway_time };
        if act_gateway_time.is_null() {
            None
        } else {
            Some(Time(act_gateway_time, PhantomData))
        }
    }

    pub fn set_act_gateway_time(&mut self, act_gateway_time: Option<&'a Time>) {
        unsafe {
            (*self.0).act_gateway_time = act_gateway_time.map_or(std::ptr::null_mut(), |v| v.0)
        }
    }
}
