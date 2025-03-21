use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
    sync::Arc,
};

use super::spelling::{self, SpellingProperties, SpellingType};
pub struct Prism;
pub struct Corrector;

type SyllableId = i32;

type Vertex = (usize, SpellingType);
type VertexQueue = BinaryHeap<Reverse<Vertex>>;

const COMPLETION_PENALTY: f64 = -0.6931471805599453; // log(0.5)
const CORRECTION_CREDIBILITY: f64 = -4.605170185988091; // log(0.01)

#[derive(Clone)]
struct EdgeProperties {
    spelling_properties: SpellingProperties,
    is_correction: bool,
}

impl Default for EdgeProperties {
    fn default() -> Self {
        Self {
            spelling_properties: SpellingProperties::default(),
            is_correction: false,
        }
    }
}

impl EdgeProperties {
    pub(crate) fn new(spelling_properties: SpellingProperties) -> Self {
        Self {
            spelling_properties,  // 使用传入的 SpellingProperties 初始化
            is_correction: false, // 默认值
        }
    }
}

type SpellingMap = BTreeMap<SyllableId, EdgeProperties>;
type VertexMap = BTreeMap<usize, SpellingType>;
type EndVertexMap = BTreeMap<usize, SpellingMap>;
type EdgeMap = BTreeMap<usize, EndVertexMap>;

type SpellingPropertiesList = Vec<Arc<EdgeProperties>>;
type SpellingIndex = BTreeMap<SyllableId, SpellingPropertiesList>;
type SpellingIndices = BTreeMap<usize, SpellingIndex>;

pub struct SyllableGraph {
    input_lenth: usize,
    interpreted_length: usize,
    vertices: VertexMap,
    edges: EdgeMap,
    indices: SpellingIndices,
}

impl Default for SyllableGraph {
    fn default() -> Self {
        Self {
            input_lenth: 0,
            interpreted_length: 0,
            vertices: VertexMap::new(),
            edges: EdgeMap::new(),
            indices: SpellingIndices::new(),
        }
    }
}

pub struct Syllabifier {
    delimiters: String,
    enable_completion: bool,
    strict_spelling: bool,
    corrector: Option<Box<Corrector>>,
}

impl Default for Syllabifier {
    fn default() -> Self {
        Syllabifier::new()
    }
}

impl Syllabifier {
    pub fn new() -> Self {
        Syllabifier {
            delimiters: String::new(),
            enable_completion: false,
            strict_spelling: false,
            corrector: None,
        }
    }
    pub fn from(delimiters: String, enable_completion: bool, strict_spelling: bool) -> Self {
        Syllabifier {
            delimiters,
            enable_completion,
            strict_spelling,
            corrector: None,
        }
    }

    // 公开方法
    pub fn build_syllable_graph(
        input: String,
        prism: &mut Prism,
        graph: &mut SyllableGraph,
    ) -> i32 {
        // 方法实现
        0 // 返回一个示例整数
    }

    // 公开方法
    pub fn enable_correction(&mut self, corrector: Option<Box<Corrector>>) {
        self.corrector = corrector;
    }
    // 其他 protected 方法
    pub(crate) fn check_overlapped_spellings(
        &self,
        graph: &mut SyllableGraph,
        start: usize,
        end: usize,
    ) {
        // 方法实现
    }

    pub(crate) fn transpose(&self, graph: &mut SyllableGraph) {
        for (start, end_map) in &graph.edges {
            let index = &mut graph.indices.entry(*start).or_default();
            for (_, end) in end_map.iter().rev() {
                for (spelling_first, spelling_second) in end {
                    index
                        .entry(*spelling_first as SyllableId)
                        .or_insert_with(Vec::new)
                        .push(Arc::new(spelling_second.clone()));
                }
            }
        }
    }
}
