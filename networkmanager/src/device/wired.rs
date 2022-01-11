// SPDX-License-Identifier: MPL-2.0

use crate::interface::device::wired::WiredDeviceProxy;
use std::ops::Deref;

pub struct WiredDevice<'a>(WiredDeviceProxy<'a>);

impl<'a> Deref for WiredDevice<'a> {
	type Target = WiredDeviceProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> From<WiredDeviceProxy<'a>> for WiredDevice<'a> {
	fn from(device: WiredDeviceProxy<'a>) -> Self {
		WiredDevice(device)
	}
}
