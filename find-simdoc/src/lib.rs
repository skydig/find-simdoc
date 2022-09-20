//! Time- and memory-efficient all-pair similarity searches in documents.
//! A more detailed description can be found on the [project page](https://github.com/legalforce-research/find-simdoc).
//!
//! # Problem definition
//!
//! - Input
//!   - List of documents
//!   - Distance function
//!   - Radius threshold
//! - Output
//!   - All pairs of similar document ids
//!
//! # Features
//!
//! - **Easy to use:** This software supports all essential steps of document similarity search,
//! from feature extraction to output of similar pairs.
//! Therefore, you can immediately try the fast all-pair similarity search using your document files.
//! - **Flexible tokenization:** You can specify any delimiter when splitting words in tokenization for feature extraction.
//! This can be useful in languages where multiple definitions of words exist, such as Japanese or Chinese.
//! - **Time efficiency:** The time complexity is *linear* over the numbers of input documents and output results,
//! based on the idea of the [sketch sorting approach](https://proceedings.mlr.press/v13/tabei10a.html).
//! - **Memory efficiency:** The memory complexity is *linear* over the numbers of input documents,
//! and the actual memory usage is also very low thanks to compact binary sketches by locality sensitive hashing.
//!
//! # Search steps
//!
//! 1. Extract features from documents
//!    - Set representation of character or word ngrams
//!    - Tfidf-weighted vector representation of character or word ngrams
//! 2. Convert the features into binary sketches through locality sensitive hashing
//!    - [1-bit minwise hashing](https://dl.acm.org/doi/abs/10.1145/1772690.1772759) for the Jaccard similarity
//!    - [Simplified simhash](https://dl.acm.org/doi/10.1145/1242572.1242592) for the Cosine similarity
//! 3. Search for similar sketches in the Hamming space using a modified variant of the [sketch sorting approach](https://proceedings.mlr.press/v13/tabei10a.html)
#![deny(missing_docs)]

pub mod cosine;
pub mod errors;
pub mod feature;
pub mod jaccard;
pub mod lsh;
pub mod tfidf;

pub(crate) mod shingling;

pub use cosine::CosineSearcher;
pub use jaccard::JaccardSearcher;
