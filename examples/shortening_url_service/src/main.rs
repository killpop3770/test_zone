//! ## Task Description
//!
//! The goal is to develop a backend service for shortening URLs using CQRS
//! (Command Query Responsibility Segregation) and ES (Event Sourcing)
//! approaches. The service should support the following features:
//!
//! ## Functional Requirements
//!
//! ### Creating a short link with a random slug
//!
//! The user sends a long URL, and the service returns a shortened URL with a
//! random slug.
//!
//! ### Creating a short link with a predefined slug
//!
//! The user sends a long URL along with a predefined slug, and the service
//! checks if the slug is unique. If it is unique, the service creates the short
//! link.
//!
//! ### Counting the number of redirects for the link
//!
//! - Every time a user accesses the short link, the click count should
//!   increment.
//! - The click count can be retrieved via an API.
//!
//! ### CQRS+ES Architecture
//!
//! CQRS: Commands (creating links, updating click count) are separated from
//! queries (retrieving link information).
//!
//! Event Sourcing: All state changes (link creation, click count update) must be
//! recorded as events, which can be replayed to reconstruct the system's state.
//!
//! ### Technical Requirements
//!
//! - The service must be built using CQRS and Event Sourcing approaches.
//! - The service must be possible to run in Rust Playground (so no database like
//!   Postgres is allowed)
//! - Public API already written for this task must not be changed (any change to
//!   the public API items must be considered as breaking change).
//! - Event Sourcing should be actively utilized for implementing logic, rather
//!   than existing without a clear purpose.

#![allow(unused_variables, dead_code)]
use std::collections::HashMap;
use crate::queries::QueryHandler;
use commands::CommandHandler;
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// All possible errors of the [`UrlShortenerService`].
#[derive(Debug, PartialEq)]
pub enum ShortenerError {
    /// This error occurs when an invalid [`Url`] is provided for shortening.
    InvalidUrl,

    /// This error occurs when an attempt is made to use a slug (custom alias)
    /// that already exists.
    SlugAlreadyInUse,

    /// This error occurs when the provided [`Slug`] does not map to any existing
    /// short link.
    SlugNotFound,
}

/// A unique string (or alias) that represents the shortened version of the
/// URL.
#[derive(Clone, Debug, PartialEq)]
pub struct Slug(pub String);

/// The original URL that the short link points to.
#[derive(Clone, Debug, PartialEq)]
pub struct Url(pub String);

/// Shortened URL representation.
#[derive(Debug, Clone, PartialEq)]
pub struct ShortLink {
    /// A unique string (or alias) that represents the shortened version of the
    /// URL.
    pub slug: Slug,

    /// The original URL that the short link points to.
    pub url: Url,
}

/// Statistics of the [`ShortLink`].
#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    /// [`ShortLink`] to which this [`Stats`] are related.
    pub link: ShortLink,

    /// Count of redirects of the [`ShortLink`].
    pub redirects: u64,
}

/// Commands for CQRS.
pub mod commands {
    use super::{ShortLink, ShortenerError, Slug, Url};

    /// Trait for command handlers.
    pub trait CommandHandler {
        /// Creates a new short link. It accepts the original url and an
        /// optional [`Slug`]. If a [`Slug`] is not provided, the service will generate
        /// one. Returns the newly created [`ShortLink`].
        ///
        /// ## Errors
        ///
        /// See [`ShortenerError`].
        fn handle_create_short_link(
            &mut self,
            url: Url,
            slug: Option<Slug>,
        ) -> Result<ShortLink, ShortenerError>;

        /// Processes a redirection by [`Slug`], returning the associated
        /// [`ShortLink`] or a [`ShortenerError`].
        fn handle_redirect(
            &mut self,
            slug: Slug,
        ) -> Result<ShortLink, ShortenerError>;

        /// Changes [Url] of a [ShortLink] with a provided [Slug].
        fn handle_change_short_link(
            &mut self,
            slug: Slug,
            new_url: Url
        ) -> Result<ShortLink, ShortenerError>;
    }
}

/// Queries for CQRS
pub mod queries {
    use super::{ShortenerError, Slug, Stats};

    /// Trait for query handlers.
    pub trait QueryHandler {
        /// Returns the [`Stats`] for a specific [`ShortLink`], such as the
        /// number of redirects (clicks).
        ///
        /// [`ShortLink`]: super::ShortLink
        fn get_stats(&self, slug: Slug) -> Result<Stats, ShortenerError>;
    }
}

#[derive(Debug, Clone)]
enum Event {
    LinkCreated { id: Uuid, long_url: String, short_url: String, },
    LinkClicked { id: String, },
}


/// CQRS and Event Sourcing-based service implementation
pub struct UrlShortenerService {
    // TODO: add needed fields
    short_links: HashMap<String, ShortLink>,
    stats: HashMap<String, Stats>,
    long_urls_already_in_use: Vec<String>,
    events: Vec<Event>,
    next_uuid: Uuid,
}

impl UrlShortenerService {
    /// Creates a new instance of the service
    pub fn new() -> Self {
        Self {
            short_links: HashMap::new(),
            stats: HashMap::new(),
            long_urls_already_in_use: Vec::new(),
            events: Vec::new(),
            next_uuid: Uuid::new_v4(),
        }
    }

    pub fn rebuild_service_state_by_events(&mut self) {
        self.short_links.clear();
        self.stats.clear();

        for event in &self.events {
            match event {
                Event::LinkCreated { id, long_url, short_url } => {
                    let short_link = ShortLink{slug: Slug(short_url.clone()), url: Url(long_url.clone())};
                    self.short_links.insert(short_url.clone(), short_link.clone());
                    self.stats.insert(short_url.clone(), Stats { link: short_link, redirects: 0 });
                },
                Event::LinkClicked { id } => {
                    if let Some(stats) = self.stats.get_mut(&id.to_string()) {
                        stats.redirects+=1;
                    }
                },
            }
        }
    }
}

impl commands::CommandHandler for UrlShortenerService {
    fn handle_create_short_link(
        &mut self,
        url: Url,
        slug: Option<Slug>,
    ) -> Result<ShortLink, ShortenerError> {
        let id = self.next_uuid.clone();
        self.next_uuid = Uuid::new_v4();
        let slug_to_use = if let Some(slug) = slug {
            if self.short_links.contains_key(&slug.0) {
                return Err(ShortenerError::SlugAlreadyInUse);
            }
            slug.0
        } else {
            if url.0.is_empty() || url.0.trim().is_empty() {
                return Err(ShortenerError::InvalidUrl);
            }
            if self.long_urls_already_in_use.contains(&url.0) {
                return Err(ShortenerError::InvalidUrl);
            }
            let mut hasher = Sha256::new();
            hasher.update(url.0.as_bytes());
            let result = hasher.finalize();
            let mut hex_string = String::with_capacity(result.len() * 2);
            for byte in result {
                hex_string.push_str(&format!("{:02x}", byte));
            }
            let short_url = hex_string[..10].to_string();
            short_url
        };

        let short_link = ShortLink { 
            slug: Slug(slug_to_use.clone()), 
            url: url.clone(), 
        };
        let stats = Stats {
            link: short_link.clone(),
            redirects: 0
        };

        let event = Event::LinkCreated { id, long_url: url.0.clone(), short_url: slug_to_use.clone()};
        self.events.push(event);
        self.short_links.insert(slug_to_use.clone(), short_link.clone());
        self.stats.insert(slug_to_use.clone(), stats);
        self.long_urls_already_in_use.push(url.0);
        Ok(short_link)
    }

    fn handle_redirect(
        &mut self,
        slug: Slug,
    ) -> Result<ShortLink, ShortenerError> {
        let short_link = self.short_links.get(&slug.0).ok_or(ShortenerError::SlugNotFound)?;
        let stats = self.stats.get_mut(&slug.0).ok_or(ShortenerError::SlugNotFound)?;
        let event = Event::LinkClicked { id: short_link.slug.0.to_string() };
        self.events.push(event);
        stats.redirects += 1;
        Ok(short_link.clone())
    }
    
    fn handle_change_short_link(
        &mut self,
        slug: Slug,
        new_url: Url
    ) -> Result<ShortLink, ShortenerError> {
        if new_url.0.is_empty() || new_url.0.trim().is_empty() {
            return Err(ShortenerError::InvalidUrl);
        }
        let short_link = self.short_links.get_mut(&slug.0).ok_or(ShortenerError::SlugNotFound)?;
        short_link.url = new_url;
        Ok(short_link.clone())
    }
}

impl queries::QueryHandler for UrlShortenerService {
    fn get_stats(&self, slug: Slug) -> Result<Stats, ShortenerError> {
        match self.stats.get(&slug.0) {
            Some(stats) => Ok(stats.clone()),
            None => Err(ShortenerError::SlugNotFound),
        }
    }
}

// Introduce functionality to allow updating the URL of an existing short link. 
// This new functionality should be reflected in the CommandHandler trait.  
// Update the CommandHandler trait to include the following method:  


fn main() -> () {

    let mut service = UrlShortenerService::new();

    let short_link1 = service.handle_create_short_link(Url("https://example.com/123".to_string()), None).unwrap(); 
    println!("Ok when create link1: {:?}", short_link1);

    let short_link2 = service.handle_create_short_link(Url("https://example.com/456".to_string()), None).unwrap(); 
    match service.handle_change_short_link(short_link2.slug, Url("https://example.com/654".to_string())){
            Ok(short_link) => println!("Ok when create link2: {:?}", short_link),
            Err(err) => println!("Error when create link2: {:?}", err),
    }

    match service.handle_create_short_link(Url("https://example.com/123".to_string()), None){
        Ok(short_link) => println!("Ok when create link2: {:?}", short_link),
        Err(err) => println!("Error when create link2: {:?}", err),
    }

    match service.handle_create_short_link(Url("https://google.com/12345".to_string()), Some(Slug("https://ya.ru/".to_string()))) {
        Ok(short_link) => println!("Ok when create link3: {:?}", short_link),
        Err(err) => println!("Error when create link3: {:?}", err),
    }

    match service.handle_create_short_link(Url("   ".to_string()), None){
        Ok(short_link) => println!("Ok when create link4: {:?}", short_link),
        Err(err) => println!("Error when create link4: {:?}", err),
    }

    let short_link5 = service.handle_create_short_link(Url("https://example.com/012/345/678/".to_string()), None).unwrap(); 
    println!("Ok when create link5: {:?}", short_link5);

    service.handle_redirect(short_link1.slug.clone()).unwrap();
    let stats = service.get_stats(short_link1.slug.clone()).unwrap();
    println!("Stats for {}: {:?}", short_link1.slug.0, stats);

    service.handle_redirect(short_link5.slug.clone()).unwrap();
    service.handle_redirect(short_link5.slug.clone()).unwrap();
    service.handle_redirect(short_link5.slug.clone()).unwrap();
    let stats = service.get_stats(short_link5.slug.clone()).unwrap();
    println!("Stats for {}: {:?}", short_link5.slug.0, stats);

    service.rebuild_service_state_by_events();

    let stats = service.get_stats(short_link1.slug.clone()).unwrap();
    println!("Stats for {}: {:?}", short_link1.slug.0, stats);
    
    let stats = service.get_stats(short_link5.slug.clone()).unwrap();
    println!("Stats for {}: {:?}", short_link5.slug.0, stats);
}