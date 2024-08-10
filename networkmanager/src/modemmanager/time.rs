//! # D-Bus interface proxy for: `org.freedesktop.ModemManager1.Modem.Time`
//!
//! This code was generated by `zbus-xmlgen` `4.1.0` from D-Bus introspection data.
//! Source: `Interface '/org/freedesktop/ModemManager1/Modem/4' from service 'org.freedesktop.ModemManager1' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//! This type implements the [D-Bus standard interfaces], (`org.freedesktop.DBus.*`) for which the
//! following zbus API can be used:
//!
//! * [`zbus::fdo::PropertiesProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PeerProxy`]
//!
//! Consequently `zbus-xmlgen` did not generate code for the above interfaces.
//!
//! [Writing a client proxy]: https://dbus2.github.io/zbus/client.html
//! [D-Bus standard interfaces]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces,
use zbus::proxy;
#[proxy(
	interface = "org.freedesktop.ModemManager1.Modem.Time",
	default_service = "org.freedesktop.ModemManager1",
	default_path = "/org/freedesktop/ModemManager1/Modem/4"
)]
trait Time {
	/// GetNetworkTime method
	fn get_network_time(&self) -> zbus::Result<String>;

	/// NetworkTimeChanged signal
	#[zbus(signal)]
	fn network_time_changed(&self, time: &str) -> zbus::Result<()>;

	/// NetworkTimezone property
	#[zbus(property)]
	fn network_timezone(
		&self,
	) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;
}
