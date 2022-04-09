//! Want to have your API documented with OpenAPI? But you dont want to see the
//! trouble with manual yaml or json tweaking? Would like it to be so easy that it would almost
//! be like utopic? Don't worry utoipa is just there to fill this gap. It aims to do if not all then
//! the most of heavy lifting for you enabling you to focus writing the actual API logic instead of
//! documentation. It aims to be *minimal*, *simple* and *fast*. It uses simple proc macros which
//! you can use to annotate your code to have items documented.
//!
//! Utoipa crate provides autogenerated OpenAPI documentation for Rust REST APIs. It treats
//! code first appoach as a first class citizen and simplifies API documentation by providing
//! simple macros for generating the documentation from your code.
//!
//! It also contains Rust types of OpenAPI spec allowing you to write the OpenAPI spec only using
//! Rust if autogeneration is not your flavor or does not fit your purpose.
//!
//! Long term goal of the library is to be the place to go when OpenAPI documentation is needed in Rust
//! codebase.
//!
//! Utoipa is framework agnostic and could be used together with any web framework or even without one. While
//! being portable and standalone one of it's key aspects is simple integration with web frameworks.
//!
//! Currently utoipa provides simple integration with actix-web framework but is not limited to the actix-web
//! framework. All functionalities are not restricted to any specific framework.
//!
//! # Choose your flavor and document your API with ice cold IPA
//!
//! Existing [examples](https://github.com/juhaku/utoipa/tree/master/examples) for following frameworks:
//!
//! * **actix-web**
//! * **warp**
//! * **tide**
//!
//! **actix-web** has support support for actix specific parsing via **actix_extras** feature.
//!
//! Even if there is no example for your favourite framework `utoipa` can be used with any
//! web framework which supports decorating functions with macros similarly to **warp** and **tide** examples.
//!
//! # What's up with the word play?
//!
//! The name comes from words `utopic` and `api` where `uto` is the first three letters of _utopic_
//! and the `ipa` is _api_ reversed. Aaand... `ipa` is also awesome type of beer.
//!
//! # Features
//!
//! * **default** Default enabled features are **json**.
//! * **json** Enables **serde_json** what allow to use json values in OpenAPI specification values.
//!   Thus is enabled by default.
//! * **actix_extras** Enhances actix-web intgration with being able to parse some documentation
//!   from actix web macro attributes and types. See [`utoipa::path(...)`][path] for more details.
//! * **debug** Add extra traits such as debug traits to openapi definitions and elsewhere.
//! * **chrono_types** Add support for _**chrono**_ `DateTime`, `Date` and `Duration` types. By default these types
//!   are parsed to `string` types without
//!   additional format. If you want to have formats added to the types use _chrono_with_format_ feature.
//!   This is useful because OpenAPI 3.1 spec does not have date-time formats.
//! * **chrono_types_with_format** Add support to _**chrono**_ types described above with additional `format`
//!   information type. `date-time` for `DateTime` and `date` for `Date` according
//!   [RFC3339](https://xml2rfc.ietf.org/public/rfc/html/rfc3339.html#anchor14) as `ISO-8601`.
//!
//! # Install
//!
//! Add minimal dependency declaration to Cargo.toml.
//! ```text
//! [dependencies]
//! utoipa = "0.1.1"
//! ```
//!
//! To enable more features such as use actix framework extras you could define the
//! dependency as follows.
//! ```text
//! [dependencies]
//! utoipa = { version = "0.1.1", features = ["actix_extras"] }
//! ```
//!
//! **Note!** To use `utoipa` together with Swagger UI you can use the [`utoipa-swagger-ui`][utoipa_swagger] crate.
//!
//! [utoipa_swagger]: <https://docs.rs/utoipa-swagger-ui/>
//!
//! # Examples
//!
//! Create a struct or it could be an enum also. Add `Component` derive macro to it so it can be registered
//! as a component in openapi schema.
//! ```rust
//! use utoipa::Component;
//! #[derive(Component)]
//! struct Pet {
//!    id: u64,
//!    name: String,
//!    age: Option<i32>,
//! }
//! ```
//!
//! Create an handler that would handle your business logic and add `path` proc attribute macro over it.
//! ```rust
//! mod pet_api {
//! #     use utoipa::OpenApi;
//! #     use utoipa::Component;
//! #     
//! #     #[derive(Component)]
//! #     struct Pet {
//! #       id: u64,
//! #       name: String,
//! #       age: Option<i32>,
//! #     }
//!     /// Get pet by id
//!     ///
//!     /// Get pet from database by pet id  
//!     #[utoipa::path(
//!         get,
//!         path = "/pets/{id}",
//!         responses(
//!             (status = 200, description = "Pet found succesfully", body = Pet),
//!             (status = 404, description = "Pet was not found")
//!         ),
//!         params(
//!             ("id" = u64, path, description = "Pet database id to get Pet for"),
//!         )
//!     )]
//!     async fn get_pet_by_id(pet_id: u64) -> Pet {
//!         Pet {
//!             id: pet_id,
//!             age: None,
//!             name: "lightning".to_string(),
//!         }
//!     }
//! }
//! ```
//!
//! Tie the component and the above api to the openapi schema with following `OpenApi` derive proc macro.
//! ```rust
//! # mod pet_api {
//! #     use utoipa::Component;
//! #     
//! #     #[derive(Component)]
//! #     struct Pet {
//! #       id: u64,
//! #       name: String,
//! #       age: Option<i32>,
//! #     }
//! #
//! #     /// Get pet by id
//! #     ///
//! #     /// Get pet from database by pet id  
//! #     #[utoipa::path(
//! #         get,
//! #         path = "/pets/{id}",
//! #         responses(
//! #             (status = 200, description = "Pet found succesfully", body = Pet),
//! #             (status = 404, description = "Pet was not found")
//! #         ),
//! #         params(
//! #             ("id" = u64, path, description = "Pet database id to get Pet for"),
//! #         )
//! #     )]
//! #     async fn get_pet_by_id(pet_id: u64) -> Pet {
//! #         Pet {
//! #             id: pet_id,
//! #             age: None,
//! #             name: "lightning".to_string(),
//! #         }
//! #     }
//! # }
//! # use utoipa::Component;
//! #
//! # #[derive(Component)]
//! # struct Pet {
//! #   id: u64,
//! #   name: String,
//! #   age: Option<i32>,
//! # }
//! # use utoipa::OpenApi;
//! #[derive(OpenApi)]
//! #[openapi(handlers(pet_api::get_pet_by_id), components(Pet))]
//! struct ApiDoc;
//!
//! println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
//! ```
//! # Go beyond the surface
//!
//! * See how to serve OpenAPI doc via Swagger UI check [`utoipa-swagger-ui`][utoipa_swagger] crate for more details.
//! * Browse to [examples](https://github.com/juhaku/utoipa/tree/master/examples) for more comprehensinve examples.
//! * Modify generated OpenAPI at runtime check [`Modify`] trait for more details.
//! * More about OpenAPI security in [security documentation][security].
//!
//! [path]: attr.path.html
//!
//! [security]: openapi/security/index.html

pub mod openapi;

pub use utoipa_gen::*;

/// Trait for implementing OpenAPI specification in Rust.
///
/// This trait is derivable and can be used with `#[derive]` attribute. The derived implementation
/// will use Cargo provided environment variables to implement the default information. For a details of
/// `#[derive(Component)]` refer to [derive documentation][derive].
///
/// # Examples
///
/// Below is derived example of `OpenApi`.
/// ```rust
/// use utoipa::OpenApi;
/// #[derive(OpenApi)]
/// #[openapi(handlers())]
/// struct OpenApiDoc;
/// ```
///
/// This manual `OpenApi` trait implementation is approximately equal to the above derived one except the derive
/// implementation will by default use the Cargo environment variables to set defaults for *application name,
/// version, application description, license, author name & email*.
///
///```rust
/// struct OpenApiDoc;
///
/// impl utoipa::OpenApi for OpenApiDoc {
///     fn openapi() -> utoipa::openapi::OpenApi {
///         use utoipa::{Component, Path};
///         utoipa::openapi::OpenApiBuilder::new()
///             .info(utoipa::openapi::InfoBuilder::new()
///                 .title("application name")
///                 .version("version")
///                 .description(Some("application description"))
///                 .license(Some(utoipa::openapi::License::new("MIT")))
///                 .contact(
///                     Some(utoipa::openapi::ContactBuilder::new()
///                         .name(Some("author name"))
///                         .email(Some("author email")).build()),
///             ).build())
///             .paths(utoipa::openapi::path::Paths::new())
///             .components(Some(utoipa::openapi::Components::new()))
///             .build()
///     }
/// }
/// ```
/// [derive]: derive.OpenApi.html
pub trait OpenApi {
    fn openapi() -> openapi::OpenApi;
}

/// Trait for implementing OpenAPI Schema object.
///
/// This trait is deriveable and can be used with `[#derive]` attribute. For a details of
/// `#[derive(Component)]` refer to [derive documentation][derive].
///
/// [derive]: derive.Component.html
///
/// # Examples
///
/// Use `#[derive]` to implement `Component` trait.
/// ```rust
/// # use utoipa::Component;
/// #[derive(Component)]
/// #[component(example = json!({"name": "bob the cat", "id": 1}))]
/// struct Pet {
///     id: u64,
///     name: String,
///     age: Option<i32>,
/// }
/// ```
///
/// Following manual implementation is equal to above derive one.
/// ```rust
/// # struct Pet {
/// #     id: u64,
/// #     name: String,
/// #     age: Option<i32>,
/// # }
/// #
/// impl utoipa::Component for Pet {
///     fn component() -> utoipa::openapi::schema::Component {
///         use utoipa::openapi::ToArray;
///         utoipa::openapi::ObjectBuilder::new()
///             .property(
///                 "id",
///                 utoipa::openapi::PropertyBuilder::new()
///                     .component_type(utoipa::openapi::ComponentType::Integer)
///                     .format(Some(utoipa::openapi::ComponentFormat::Int64)),
///             )
///             .required("id")
///             .property(
///                 "name",
///                 utoipa::openapi::Property::new(utoipa::openapi::ComponentType::String),
///             )
///             .required("name")
///             .property(
///                 "age",
///                 utoipa::openapi::PropertyBuilder::new()
///                     .component_type(utoipa::openapi::ComponentType::Integer)
///                     .format(Some(utoipa::openapi::ComponentFormat::Int32)),
///             )
///             .example(Some(serde_json::json!({
///               "name": "bob the cat", "id": 1
///             })))
///             .into()
///     }
/// }
/// ```
pub trait Component {
    fn component() -> openapi::schema::Component;
}

/// Trait for implementing OpenAPI PathItem object with path.
///
/// This trait is implemented via [`#[utoipa::path(...)]`][derive] attribute macro and there
/// is no need to implement this trait manually.
///
/// # Examples
///
/// Use `#[utoipa::path(..)]` to implement Path trait
/// ```rust
/// # struct Pet {
/// #   id: u64,
/// #   name: String,
/// # }
/// #
/// #
/// /// Get pet by id
/// ///
/// /// Get pet from database by pet database id  
/// #[utoipa::path(
///     get,
///     path = "/pets/{id}",
///     responses(
///         (status = 200, description = "Pet found succesfully", body = Pet),
///         (status = 404, description = "Pet was not found")
///     ),
///     params(
///         ("id" = u64, path, description = "Pet database id to get Pet for"),
///     )
/// )]
/// async fn get_pet_by_id(pet_id: u64) -> Pet {
///     Pet {
///         id: pet_id,
///         name: "lightning".to_string(),
///     }
/// }
/// ```
///
/// Example of what would manual implementation roughly look like of above `#[utoipa::path(...)]` macro.
/// ```rust
/// utoipa::openapi::PathsBuilder::new().path(
///         "/pets/{id}",
///         utoipa::openapi::PathItem::new(
///             utoipa::openapi::PathItemType::Get,
///             utoipa::openapi::path::OperationBuilder::new()
///                 .responses(
///                     utoipa::openapi::ResponsesBuilder::new()
///                         .response(
///                             "200",
///                             utoipa::openapi::ResponseBuilder::new()
///                                 .description("Pet found succesfully")
///                                 .content("application/json",
///                                     utoipa::openapi::Content::new(
///                                         utoipa::openapi::Ref::from_component_name("Pet"),
///                                     ),
///                             ),
///                         )
///                         .response("404", utoipa::openapi::Response::new("Pet was not found")),
///                 )
///                 .operation_id(Some("get_pet_by_id"))
///                 .deprecated(Some(utoipa::openapi::Deprecated::False))
///                 .summary(Some("Get pet by id"))
///                 .description(Some("Get pet by id\n\nGet pet from database by pet database id\n"))
///                 .parameter(
///                     utoipa::openapi::path::ParameterBuilder::new()
///                         .name("id")
///                         .parameter_in(utoipa::openapi::path::ParameterIn::Path)
///                         .required(utoipa::openapi::Required::True)
///                         .deprecated(Some(utoipa::openapi::Deprecated::False))
///                         .description(Some("Pet database id to get Pet for"))
///                         .schema(
///                             Some(utoipa::openapi::PropertyBuilder::new()
///                                 .component_type(utoipa::openapi::ComponentType::Integer)
///                                 .format(Some(utoipa::openapi::ComponentFormat::Int64))),
///                         ),
///                 )
///                 .tag("pet_api"),
///         ),
///     );
/// ```
///
/// [derive]: attr.path.html
pub trait Path {
    fn path() -> &'static str;

    fn path_item(defalt_tag: Option<&str>) -> openapi::path::PathItem;
}

/// Trait that allows OpenApi modification at runtime.
///
/// Implement this trait if you wish to modify the OpenApi at runtime before it is being consumed
/// *(Before `utoipa::OpenApi::openapi()` function returns)*.
/// This is trait can be used to add or change already generated OpenApi spec to alter the generated
/// specification by user defined condition. For example you can add definitions that should be loaded
/// from some configuration at runtime what may not be available during compile time.
///
/// See more about [`OpenApi`][derive] derive at [derive documentation][derive].
///
/// [derive]: derive.OpenApi.html
/// [security_schema]: openapi/security/enum.SecuritySchema.html
///
/// # Examples
///
/// Add custom JWT [`SecuritySchema`][security_schema] to [`OpenApi`][`openapi::OpenApi`].
/// ```rust
/// # use utoipa::{OpenApi, Modify};
/// # use utoipa::openapi::security::{SecurityScheme, HttpBuilder, HttpAuthScheme};
/// #[derive(OpenApi)]
/// #[openapi(modifiers(&SecurityAddon))]
/// struct ApiDoc;
///
/// struct SecurityAddon;
///
/// impl Modify for SecurityAddon {
///     fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
///          openapi.components = Some(
///              utoipa::openapi::ComponentsBuilder::new()
///                  .security_schema(
///                      "api_jwt_token",
///                      SecurityScheme::Http(
///                          HttpBuilder::new()
///                              .scheme(HttpAuthScheme::Bearer)
///                              .bearer_format("JWT")
///                              .build(),
///                      ),
///                  )
///                  .build(),
///          )
///      }
/// }
/// ```
///
/// Add [OpenAPI Server Object][server] to alter the target server url. This can be used to give context
/// path for api operations.
/// ```rust
/// # use utoipa::{OpenApi, Modify};
/// # use utoipa::openapi::Server;
/// #[derive(OpenApi)]
/// #[openapi(modifiers(&ServerAddon))]
/// struct ApiDoc;
///
/// struct ServerAddon;
///
/// impl Modify for ServerAddon {
///     fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
///         openapi.servers = Some(vec![Server::new("/api")])
///     }
/// }
/// ```
///
/// [server]: https://spec.openapis.org/oas/latest.html#server-object
pub trait Modify {
    fn modify(&self, openapi: &mut openapi::OpenApi);
}
