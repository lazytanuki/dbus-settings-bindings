// SPDX-License-Identifier: MPL-2.0
//! # DBus interface proxy for: `org.freedesktop.NetworkManager.IP6Config`
//!
//! This code was generated by `zbus-xmlgen` `2.0.0` from DBus introspection data.
//! Source: `Interface '/org/freedesktop/NetworkManager/IP6Config/3' from service 'org.freedesktop.NetworkManager' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::PropertiesProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PeerProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.IP6Config")]
pub trait Ipv6Config {
	/// AddressData property
	#[dbus_proxy(property)]
	fn address_data(
		&self,
	) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

	/// Addresses property
	#[dbus_proxy(property)]
	fn addresses(&self) -> zbus::Result<Vec<(Vec<u8>, u32, Vec<u8>)>>;

	/// DnsOptions property
	#[dbus_proxy(property)]
	fn dns_options(&self) -> zbus::Result<Vec<String>>;

	/// DnsPriority property
	#[dbus_proxy(property)]
	fn dns_priority(&self) -> zbus::Result<i32>;

	/// Domains property
	#[dbus_proxy(property)]
	fn domains(&self) -> zbus::Result<Vec<String>>;

	/// Gateway property
	#[dbus_proxy(property)]
	fn gateway(&self) -> zbus::Result<String>;

	/// Nameservers property
	#[dbus_proxy(property)]
	fn nameservers(&self) -> zbus::Result<Vec<Vec<u8>>>;

	/// RouteData property
	#[dbus_proxy(property)]
	fn route_data(
		&self,
	) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

	/// Routes property
	#[dbus_proxy(property)]
	fn routes(&self) -> zbus::Result<Vec<(Vec<u8>, u32, Vec<u8>, u32)>>;

	/// Searches property
	#[dbus_proxy(property)]
	fn searches(&self) -> zbus::Result<Vec<String>>;
}
