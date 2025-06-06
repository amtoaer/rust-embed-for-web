#[cfg(feature = "include-exclude")]
use globset::{Glob, GlobMatcher};

#[derive(Debug)]
pub struct Config {
    #[cfg(feature = "include-exclude")]
    include: Vec<GlobMatcher>,
    #[cfg(feature = "include-exclude")]
    exclude: Vec<GlobMatcher>,
    gzip: bool,
    br: bool,
    preserve_source: bool,
    preserve_source_except: Vec<GlobMatcher>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            #[cfg(feature = "include-exclude")]
            include: vec![],
            #[cfg(feature = "include-exclude")]
            exclude: vec![],
            gzip: true,
            br: true,
            preserve_source: true,
            #[cfg(feature = "include-exclude")]
            preserve_source_except: vec![],
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    // Builder functions
    #[cfg(feature = "include-exclude")]
    pub fn add_include(&mut self, pattern: String) {
        self.include.push(
            Glob::new(&pattern)
                .expect("Failed to parse glob pattern for include")
                .compile_matcher(),
        );
    }

    #[cfg(feature = "include-exclude")]
    pub fn add_exclude(&mut self, pattern: String) {
        self.exclude.push(
            Glob::new(&pattern)
                .expect("Failed to parse glob pattern for exclude")
                .compile_matcher(),
        );
    }

    #[cfg(feature = "include-exclude")]
    pub fn add_preserve_source_except(&mut self, pattern: String) {
        self.preserve_source_except.push(
            Glob::new(&pattern)
                .expect("Failed to parse glob pattern for preserve source unless")
                .compile_matcher(),
        );
    }

    pub fn set_gzip(&mut self, status: bool) {
        self.gzip = status;
    }

    pub fn set_br(&mut self, status: bool) {
        self.br = status;
    }

    pub fn set_preserve_source(&mut self, status: bool) {
        self.preserve_source = status;
    }

    #[cfg(feature = "include-exclude")]
    pub fn get_includes(&self) -> &Vec<GlobMatcher> {
        &self.include
    }

    #[cfg(feature = "include-exclude")]
    pub fn get_excludes(&self) -> &Vec<GlobMatcher> {
        &self.exclude
    }

    #[cfg(feature = "include-exclude")]
    pub fn get_preserve_source_except(&self) -> &Vec<GlobMatcher> {
        &self.preserve_source_except
    }

    pub fn is_preserve_source_except(&self, path: &str) -> bool {
        #[cfg(feature = "include-exclude")]
        {
            self.preserve_source_except
                .iter()
                .any(|matcher| matcher.is_match(path))
        }
        #[cfg(not(feature = "include-exclude"))]
        {
            false
        }
    }

    /// Check if a file at some path should be included based on this config.
    ///
    /// When deciding, includes always have priority over excludes. That means
    /// you typically will list paths you want excluded, then add includes to
    /// make an exception for some subset of files.
    #[allow(unused_variables)]
    pub fn should_include(&self, path: &str) -> bool {
        #[cfg(feature = "include-exclude")]
        {
            // Includes have priority.
            self.include
            .iter()
            .any(|include| include.is_match(path))
            // If not, then we check if the file has been excluded. Any file
            // that is not explicitly excluded will be
            || !self
                .exclude
                .iter()
                .any(|exclude| exclude.is_match(path))
        }
        #[cfg(not(feature = "include-exclude"))]
        {
            true
        }
    }

    pub fn should_gzip(&self) -> bool {
        self.gzip
    }

    pub fn should_br(&self) -> bool {
        self.br
    }

    pub fn should_preserve_source(&self) -> bool {
        self.preserve_source
    }
}
