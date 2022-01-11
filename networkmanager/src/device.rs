// SPDX-License-Identifier: MPL-2.0

pub mod bluetooth;
pub mod wired;
pub mod wireless;

use crate::{
	config::{ip4::Ipv4Config, ip6::Ipv6Config},
	connection::Connection,
	interface::{
		config::{ip4::Ipv4ConfigProxy, ip6::Ipv6ConfigProxy},
		connection::ActiveConnectionProxy,
		device::DeviceProxy,
		enums::{DeviceCapabilities, DeviceState, DeviceType},
	},
};
use std::{net::Ipv4Addr, ops::Deref};
use zbus::Result;

pub struct Device<'a>(DeviceProxy<'a>);

impl<'a> Device<'a> {
	pub async fn active_connection(&self) -> Result<Connection<'a>> {
		let active_connection = self.0.active_connection().await?;
		Ok(ActiveConnectionProxy::builder(self.0.connection())
			.path(active_connection)?
			.build()
			.await?
			.into())
	}

	pub async fn available_connections(&self) -> Result<Vec<Connection<'a>>> {
		let available_connections = self.0.available_connections().await?;
		let mut out = Vec::with_capacity(available_connections.len());
		for connection in available_connections {
			let connection = ActiveConnectionProxy::builder(self.0.connection())
				.path(connection)?
				.build()
				.await?;
			out.push(connection.into());
		}
		Ok(out)
	}

	pub async fn capabilities(&self) -> Result<DeviceCapabilities> {
		self.0
			.capabilities()
			.await
			.map(DeviceCapabilities::from_bits_truncate)
	}

	pub async fn device_type(&self) -> Result<DeviceType> {
		self.0.device_type().await.map(DeviceType::from)
	}

	pub async fn ip4_address(&self) -> Result<Ipv4Addr> {
		self.0.ip4_address().await.map(Ipv4Addr::from)
	}

	pub async fn ip4_config(&self) -> Result<Ipv4Config<'a>> {
		let config = Ipv4ConfigProxy::builder(self.0.connection())
			.path(self.0.ip4_config().await?)?
			.build()
			.await?;
		Ok(Ipv4Config::from(config))
	}

	pub async fn ip6_config(&self) -> Result<Ipv6Config<'a>> {
		let config = Ipv6ConfigProxy::builder(self.0.connection())
			.path(self.0.ip6_config().await?)?
			.build()
			.await?;
		Ok(Ipv6Config::from(config))
	}

	pub async fn state(&self) -> Result<DeviceState> {
		self.0.state().await.map(DeviceState::from)
	}
}

impl<'a> Deref for Device<'a> {
	type Target = DeviceProxy<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> From<DeviceProxy<'a>> for Device<'a> {
	fn from(device: DeviceProxy<'a>) -> Self {
		Device(device)
	}
}
