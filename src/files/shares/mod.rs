// Windows and Samba shares in Files.

mod card;
mod commands;
mod kept;
#[cfg(not(test))]
mod keyring;
mod msg;
mod opening;
#[cfg(test)]
pub mod pretend;
mod rule;
mod system;
#[cfg(test)]
mod tests;
mod types;
mod window;

pub use card::view;
#[cfg(test)]
pub use kept::kept_rows;
pub use rule::{left_words, KeptRow};
pub use types::{Ask, ShareMsg, Shares};
#[cfg(test)]
pub use rule::LAPSE;
#[cfg(test)]
pub use types::Tried;

#[cfg(test)]
pub(super) const SOURCE: &[&str] = &[include_str!("mod.rs"), include_str!("card.rs"), include_str!("commands.rs"), include_str!("kept.rs"), include_str!("keyring.rs"), include_str!("msg.rs"), include_str!("opening.rs"), include_str!("pretend.rs"), include_str!("rule.rs"), include_str!("system.rs"), include_str!("types.rs"), include_str!("window.rs")];
