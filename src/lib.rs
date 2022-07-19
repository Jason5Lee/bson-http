pub mod client;
pub mod server;

/// We use this content-type according to the ASP.NET design.
///     https://docs.microsoft.com/en-us/aspnet/web-api/overview/formats-and-model-binding/bson-support-in-web-api-21
const BSON_CONTENT_TYPE: &str = "application/bson";
