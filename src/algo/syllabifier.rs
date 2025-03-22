use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
    sync::Arc,
};

use crate::dict::corrector::Corrector;

use super::spelling::{SpellingProperties, SpellingType};

pub type SyllableId = i32;

#[derive(Clone)]
struct EdgeProperties {
    spelling_properties: RefCell<SpellingProperties>,
    is_correction: bool,
}

impl Default for EdgeProperties {
    fn default() -> Self {
        Self {
            spelling_properties: RefCell::default(),
            is_correction: false,
        }
    }
}

impl EdgeProperties {
    pub(crate) fn new(spelling_properties: RefCell<SpellingProperties>) -> Self {
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

#[derive(Debug)]
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

impl From<(String, bool, bool)> for Syllabifier {
    fn from(value: (String, bool, bool)) -> Self {
        Self {
            delimiters: value.0,
            enable_completion: value.1,
            strict_spelling: value.2,
            corrector: None,
        }
    }
}

type Vertex = (usize, SpellingType);
type VertexQueue = BinaryHeap<Reverse<Vertex>>;

const COMPLETION_PENALTY: f64 = -0.6931471805599453; // log(0.5)
const CORRECTION_CREDIBILITY: f64 = -4.605170185988091; // log(0.01)

impl Syllabifier {
    pub fn new() -> Self {
        Syllabifier {
            delimiters: String::new(),
            enable_completion: false,
            strict_spelling: false,
            corrector: None,
        }
    }

    // TODO
    /*  pub fn build_syllable_graph(
            input: String,
            prism: &mut Prism,
            graph: &mut SyllableGraph,
        ) -> i32 {
            if input.is_empty() {
                return 0;
            };

            let mut farthest: usize = 0;
            let mut queue: VertexQueue = BinaryHeap::new();
            queue.push(Reverse((0, SpellingType::NormalSpelling))); // start

            while let Some(Reverse(vertex)) = queue.pop() {
                let current_pos = vertex.0;

                // record a visit to the vertex
                if !graph.vertices.contains_key(&current_pos) {
                    graph.vertices.insert(current_pos, vertex.1); // preferred spelling type comes first
                } else {
                    // *graph.vertices.get_mut(&current_pos).unwrap() =
                    //     std::cmp::min(vertex.1, *graph.vertices.get(&current_pos).unwrap());
                    continue; // discard worse spelling types
                }
                if current_pos > farthest {
                    farthest = current_pos;
                    // 日志相关无法直接迁移 DLOG(INFO) << "current_pos: " << current_pos;

                    // see where we can go by advancing a syllable
                    todo!("依赖 dict/prism {}", farthest);
                }
            }

            todo!(
                "build_syllable_graph() 依赖其他代码，尚未实现{:?}， {}，{} ",
                prism,
                COMPLETION_PENALTY,
                CORRECTION_CREDIBILITY
            );
        }

        pub fn enable_correction(&mut self, corrector: Option<Box<Corrector>>) {
            self.corrector = corrector;
        }
    */
    /*pub(crate) fn check_overlapped_spellings0(
        &self,
        mut graph: Option<SyllableGraph>,
        start: usize,
        end: usize,
    ) {
        if let Some(mut graph) = graph {
            // 遍历 indices 中的每个顶点
            for (vertex_id, spelling_index) in &mut graph.indices {
                // 遍历每个 SyllableId 对应的 SpellingPropertiesList
                for (syllable_id, spelling_properties_list) in spelling_index {
                    // 遍历每个 Arc<EdgeProperties>
                    for edge_properties_arc in spelling_properties_list {
                        // 获取 Arc 的可变引用
                        if let Some(edge_properties) = Arc::get_mut(edge_properties_arc) {
                            // 检查 end_pos 是否在指定的范围内

                            // 修改 credibility 的值
                            edge_properties.spelling_properties.credibility = 1.0;
                        }
                    }
                }
            }
        }
    }*/

    pub(crate) fn check_overlapped_spellings(
        &self,
        graph: Option<SyllableGraph>,
        start: usize,
        end: usize,
    ) {
        const PENALTY_FOR_AMBIGUOUS_SYLLABLE: f64 = -23.025850929940457; // log(1e-10)

        let Some(graph) = graph.as_ref() else {
            return;
        };

        let Some(y_end_vertices) = graph.edges.get(&start) else {
            return;
        };

        // if "Z" = "YX", mark the vertex between Y and X an ambiguous syllable joint
        // enumerate Ys
        for (joint, _) in y_end_vertices.iter() {
            if *joint >= end {
                break;
            }

            // test X
            if let Some(x_end_vertices) = graph.edges.get(joint) {
                for (x_key, x_value) in x_end_vertices.iter() {
                    if x_key < &end {
                        continue;
                    }
                    if x_key == &end {
                        // discourage syllables at an ambiguous joint
                        // bad cases include pinyin syllabification "niju'ede"
                        for spelling in x_value.values() {
                            spelling
                                .spelling_properties
                                .try_borrow_mut()
                                .unwrap()
                                .credibility += PENALTY_FOR_AMBIGUOUS_SYLLABLE;
                        }
                    }
                }
            }
        }

        todo!(" 方法还没实现呢{},{}", start, end);
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
