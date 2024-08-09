// SPDX-License-Identifier: MPL-2.0

use super::Device;
use crate::interface::device::{modem::ModemDeviceProxy, DeviceProxy};
use std::ops::Deref;
use zbus::Result;

#[derive(Debug)]
pub struct ModemDevice<'a>(ModemDeviceProxy<'a>);

impl<'a> ModemDevice<'a> {
	pub async fn upcast(&'a self) -> Result<Device<'a>> {
		DeviceProxy::builder(self.0.inner().connection())
			.path(self.0.inner().path())?
			.build()
			.await
			.map(Device::from)
	}
}

impl<'a> Deref for ModemDevice<'a> {
	type Target = ModemDeviceProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> From<ModemDeviceProxy<'a>> for ModemDevice<'a> {
	fn from(device: ModemDeviceProxy<'a>) -> Self {
		ModemDevice(device)
	}
}
