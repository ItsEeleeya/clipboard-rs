pub mod common;
mod platform;
pub use common::{ClipboardContent, ClipboardHandler, ContentFormat, Result, RustImageData};
pub use image::imageops::FilterType;
pub use platform::{ClipboardContext, ClipboardWatcherContext, WatcherShutdown};
/// Trait representing a clipboard.
pub trait Clipboard: Send {
	/// Get all formats of the current content in the clipboard.
	///
	/// # Returns
	///
	/// A `Result` containing a vector of strings representing the available formats.
	fn available_formats(&self) -> Result<Vec<String>>;

	/// Check if the clipboard has content in the specified format.
	///
	/// # Parameters
	///
	/// - `format`: The format to check for.
	///
	/// # Returns
	///
	/// `true` if the clipboard has content in the specified format, `false` otherwise.
	fn has(&self, format: ContentFormat) -> bool;

	/// Clear the clipboard.
	///
	/// # Returns
	///
	/// A `Result` indicating success or failure.
	fn clear(&self) -> Result<()>;

	/// Get the data in the specified format in the clipboard as a byte array.
	///
	/// # Parameters
	///
	/// - `format`: The format of the data to retrieve.
	///
	/// # Returns
	///
	/// A `Result` containing a vector of bytes representing the data.
	fn get_buffer(&self, format: &str) -> Result<Vec<u8>>;

	/// Get plain text content in the clipboard as a string.
	///
	/// # Returns
	///
	/// A `Result` containing the plain text content as a string.
	fn get_text(&self) -> Result<String>;

	/// Get the rich text content in the clipboard as a string.
	///
	/// # Returns
	///
	/// A `Result` containing the rich text content as a string.
	fn get_rich_text(&self) -> Result<String>;

	/// Get the HTML format content in the clipboard as a string.
	///
	/// # Returns
	///
	/// A `Result` containing the HTML content as a string.
	fn get_html(&self) -> Result<String>;

	/// Get the image content in the clipboard.
	///
	/// # Returns
	///
	/// A `Result` containing the image data.
	fn get_image(&self) -> Result<RustImageData>;

	/// Get the file paths in the clipboard.
	///
	/// # Returns
	///
	/// A `Result` containing a vector of file paths.
	fn get_files(&self) -> Result<Vec<String>>;

	/// Get the content in the specified formats from the clipboard.
	///
	/// # Parameters
	///
	/// - `formats`: The formats to retrieve.
	///
	/// # Returns
	///
	/// A `Result` containing a vector of clipboard contents.
	fn get(&self, formats: &[ContentFormat]) -> Result<Vec<ClipboardContent>>;

	/// Set the data in the specified format in the clipboard as a byte array.
	///
	/// # Parameters
	///
	/// - `format`: The format of the data to set.
	/// - `buffer`: The byte array representing the data.
	///
	/// # Returns
	///
	/// A `Result` indicating success or failure.
	fn set_buffer(&self, format: &str, buffer: Vec<u8>) -> Result<()>;

	/// Set the plain text content in the clipboard.
	///
	/// # Parameters
	///
	/// - `text`: The plain text content to set.
	///
	/// # Returns
	///
	/// A `Result` indicating success or failure.
	fn set_text(&self, text: String) -> Result<()>;

	/// Set the rich text content in the clipboard.
	///
	/// # Parameters
	///
	/// - `text`: The rich text content to set.
	///
	/// # Returns
	///
	/// A `Result` indicating success or failure.
	fn set_rich_text(&self, text: String) -> Result<()>;

	/// Set the HTML content in the clipboard.
	///
	/// # Parameters
	///
	/// - `html`: The HTML content to set.
	///
	/// # Returns
	///
	/// A `Result` indicating success or failure.
	fn set_html(&self, html: String) -> Result<()>;

	/// Set the image content in the clipboard.
	///
	/// # Parameters
	///
	/// - `image`: The image data to set.
	///
	/// # Returns
	///
	/// A `Result` indicating success or failure.
	fn set_image(&self, image: RustImageData) -> Result<()>;

	/// Set the file paths in the clipboard.
	///
	/// # Parameters
	///
	/// - `files`: The file paths to set.
	///
	/// # Returns
	///
	/// A `Result` indicating success or failure.
	fn set_files(&self, files: Vec<String>) -> Result<()>;

	/// Set the content in the clipboard.
	///
	/// # Parameters
	///
	/// - `contents`: The clipboard contents to set.
	///
	/// # Returns
	///
	/// A `Result` indicating success or failure.
	fn set(&self, contents: Vec<ClipboardContent>) -> Result<()>;
}

/// Trait representing a clipboard watcher.
pub trait ClipboardWatcher<T: ClipboardHandler>: Send {
	/// Add a clipboard change handler, you can add multiple handlers, the handler needs to implement the `ClipboardHandler` trait.
	fn add_handler(&mut self, handler: T) -> &mut Self;

	/// Start monitoring clipboard changes, this is a blocking method, until the monitoring ends, or the `stop` method is called, so it is recommended to call it in a separate thread.
	fn start_watch(&mut self);

	/// Get the channel to stop monitoring, you can stop monitoring through this channel.
	fn get_shutdown_channel(&self) -> WatcherShutdown;
}

impl WatcherShutdown {
	/// Stop watching the clipboard.
	pub fn stop(self) {
		drop(self);
	}
}
