use eframe::egui;
use egui::accesskit::NodeId;
use petgraph::graph::{DiGraph, NodeIndex};
use std::{
    fmt::Debug,
    isize,
    marker::PhantomData,
    ops::{Index, IndexMut},
};
#[derive(Copy, Clone, Debug, PartialEq)]
struct InputPortIdx;
#[derive(Copy, Clone, Debug, PartialEq)]
struct OutputPortIdx;

#[derive(Copy, Clone, Debug, PartialEq)]
struct TypeIdx;
#[derive(Copy, Clone, Debug, PartialEq)]
struct ValueIdx;
#[derive(Copy, Clone, Debug, PartialEq)]
struct ConstIdx;
#[derive(Copy, Clone, Debug, PartialEq)]
struct NodeInstIdx;
#[derive(Copy, Clone, Debug, PartialEq)]
struct NodeIdx;
#[derive(Debug, Copy, Clone, PartialEq)]
struct AritiedTypeIdx;
#[derive(Debug, Copy, Clone, PartialEq)]
struct NodeNameIdx;
#[derive(Debug, Copy, Clone, PartialEq)]
struct SignatureIdx;
#[derive(Debug, Copy, Clone, PartialEq)]
struct FnIdx;
#[derive(Debug, Copy, Clone, PartialEq)]
struct PortAndNodeGraphIdx;

#[derive(Debug, Copy, Clone, PartialEq)]
struct StreamIdx;
#[derive(Copy, Clone, Debug, PartialEq)]
struct StringIdx<T: Copy + Clone + Debug + PartialEq>(PhantomData<T>);
#[derive(Copy, Clone, Debug, PartialEq)]
struct DateTimeIdx<T: Copy + Clone + Debug + PartialEq>(PhantomData<T>);

#[derive(Copy, Clone, Debug, PartialEq)]
struct Idx<T: Copy + Clone + PartialEq + Debug>(usize, PhantomData<T>);

#[derive(Copy, Clone, Debug, PartialEq)]
struct VecIdx<T> {
    start: usize,
    count: usize,
    _phantom: PhantomData<T>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct SetIdx<T> {
    start: usize,
    count: usize,
    _phantom: PhantomData<T>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum ManyIdx<T> {
    SetIdx(SetIdx<T>),
    VecIdx(VecIdx<T>),
}
impl From<Idx<FnIdx>> for Idx<SignatureIdx> {
    fn from(vec_idx: Idx<FnIdx>) -> Self {
        Idx(vec_idx.0, PhantomData)
    }
}
impl From<Idx<SignatureIdx>> for Idx<FnIdx> {
    fn from(vec_idx: Idx<SignatureIdx>) -> Self {
        Idx(vec_idx.0, PhantomData)
    }
}
impl From<VecIdx<FnIdx>> for VecIdx<SignatureIdx> {
    fn from(vec_idx: VecIdx<FnIdx>) -> Self {
        VecIdx {
            start: vec_idx.start,
            count: vec_idx.count,
            _phantom: PhantomData,
        }
    }
}
impl From<VecIdx<SignatureIdx>> for VecIdx<FnIdx> {
    fn from(vec_idx: VecIdx<SignatureIdx>) -> Self {
        VecIdx {
            start: vec_idx.start,
            count: vec_idx.count,
            _phantom: PhantomData,
        }
    }
}
impl From<SetIdx<FnIdx>> for SetIdx<SignatureIdx> {
    fn from(vec_idx: SetIdx<FnIdx>) -> Self {
        SetIdx {
            start: vec_idx.start,
            count: vec_idx.count,
            _phantom: PhantomData,
        }
    }
}
impl<T, C> From<VecIdx<T>> for SetIdx<C> {
    fn from(vec_idx: VecIdx<T>) -> Self {
        SetIdx {
            start: vec_idx.start,
            count: vec_idx.count,
            _phantom: PhantomData,
        }
    }
}

impl<T, C> From<SetIdx<T>> for VecIdx<C> {
    fn from(set_idx: SetIdx<T>) -> Self {
        VecIdx {
            start: set_idx.start,
            count: set_idx.count,
            _phantom: PhantomData,
        }
    }
}

type TypeVar = usize;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Port {
    InputPort(InputPortIdx),
    OutputPort(OutputPortIdx),
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct InputPort {
    owned_by: Idx<NodeIdx>,
    aritied_type: Option<Idx<AritiedType>>,
    belongs_to_lambda: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct OutputPort {
    owned_by: Idx<NodeIdx>,
    aritied_type: Option<Idx<Type>>,
    belongs_to_lambda: bool,
}

// Type system
#[derive(Copy, Clone, Debug, PartialEq)]
enum Type {
    Variable(TypeVar),
    String,
    Int,
    Float,
    DateTime,
    Vec(Idx<TypeIdx>, usize),
    Covec(Idx<TypeIdx>, usize),
    Matrix(Idx<TypeIdx>, usize, usize),
    Stream(Idx<TypeIdx>),
    Tuple(VecIdx<TypeIdx>),
    Option(Idx<TypeIdx>),
    Either(Idx<TypeIdx>, Idx<TypeIdx>),
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Either<T> {
    Left(T),
    Right(T),
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Const {
    String(Idx<StringIdx<ConstIdx>>),
    Int(isize),
    Float(f64),
    DateTime(Idx<DateTimeIdx<ConstIdx>>),
    Vec(VecIdx<ConstIdx>),
    Covec(VecIdx<ConstIdx>),
    Matrix(VecIdx<ConstIdx>, usize, usize),
    Tuple(VecIdx<ConstIdx>),
    Option(Option<ConstIdx>),
    Either(Either<ConstIdx>),
}

// Value system
#[derive(Copy, Clone, Debug, PartialEq)]
enum Value {
    String(Idx<StringIdx<ValueIdx>>),
    Int(i64),
    Float(f64),
    DateTime(chrono::DateTime<chrono::Utc>),
    Vec(VecIdx<ValueIdx>),
    Covec(VecIdx<ValueIdx>),
    Matrix(VecIdx<ValueIdx>, usize, usize),
    Stream(StreamIdx),
    Tuple(VecIdx<ValueIdx>),
    Option(Option<ValueIdx>),
    Either(Either<ValueIdx>),
}

// Arity system
#[derive(Copy, Clone, Debug, PartialEq)]
enum AritiedType {
    Single(Type),
    Many(Type),
}

#[derive(Copy, Clone, Debug)]
struct Signature {
    input: ManyIdx<AritiedTypeIdx>,
    output: VecIdx<TypeIdx>,
}

struct Env {}

struct PortAndNodeGraph {
    port_graph: DiGraph<Port, (NodeIdx, NodeIdx)>,
    node_graph: DiGraph<NodeIdx, (OutputPortIdx, InputPortIdx)>,
}

struct LambdaParams {
    inputs: VecIdx<InputPortIdx>,
    outputs: VecIdx<OutputPortIdx>,
    signature: Option<VecIdx<SignatureIdx>>,
    lambda_graph: Option<Idx<PortAndNodeGraphIdx>>,
}

struct Node {
    id: Idx<NodeId>,
    name: Idx<NodeNameIdx>,
    signature: Option<SetIdx<SignatureIdx>>,
    consts: Option<VecIdx<ConstIdx>>,
    inputs: Option<VecIdx<InputPortIdx>>,
    outputs: Option<VecIdx<OutputPortIdx>>,
    fun: Option<Idx<FnIdx>>,
    lambda: Option<LambdaParams>,
    // **********************************************
    // * kind      : Dropdown                       *
    // *              changable from node list      *
    // *                                            *
    // * name      : String Editable                *
    // * (notification signals?
    // *  should plot params?)
    // *------------------------------------------- *
    // * constant_name : Dropdown/String Editable   *
    // * constant_name : Dropdown/String Editable   *
    // * ...                                        *
    // *--------------------------------------------*
    // >> input_1 : Type                            *
    // >> input2  : Type                            *
    // >> ...                                       *
    // *                               output: Type >>
    // *                               output: Type >>
    // *                               output: Type >>
    // *                                ...         *
    // *--------------------------------------------*
    // *                                            *
    // *      (lambda 1 if exists)                  *
    // *      ****< constant   c1..cn >    *        *
    // *      *    |     |     |    |      *        *
    // *      *   c1 .. cn    o1 .. on     *        *
    // *      *                            *        *
    // >>----->> lambdainput1  : Type      *        *
    // >>----->> lambdainput2  : Type      *        *
    // >> ...                              *        *
    // *      *               output: Type >>------->>
    // *      *               output: Type >>------->>
    // *      *               output: Type >>------->>
    // *      *                       ...  >>------->>
    // *      *                            *        *
    // *      *  (Graph Editable)          *        *
    // *      *                            *        *
    // *      *     o1 .. on               *        *
    // *      *     |     |                *        *
    // *      ***< outher inputs o1..on   >*        *
    // *      * * * * * * * * * * * * * * **        *
    // *                                            *
    // *                                            *
    // *--------------------------------------------*
    // *                  lambdas...                *
    // *--------------------------------------------*
    // *                                            *
    // *      (lambda n if exists)                  *
    // *      ****< constant   c1..cn >    *        *
    // *      *    |     |     |    |      *        *
    // *      *   c1 .. cn    o1 .. on     *        *
    // *      *                            *        *
    // >>----->> lambdainput1  : Type      *        *
    // >>----->> lambdainput2  : Type      *        *
    // >> ...                              *        *
    // *      *               output: Type >>------->>
    // *      *               output: Type >>------->>
    // *      *               output: Type >>------->>
    // *      *                       ...  >>------->>
    // *      *                            *        *
    // *      *  (Graph Editable)          *        *
    // *      *                            *        *
    // *      *     o1 .. on               *        *
    // *      *     |     |                *        *
    // *      ***< outher inputs o1..on   >*        *
    // *      * * * * * * * * * * * * * * **        *
    // *                                            *
    // *                                            *
    // *--------------------------------------------*
    // * ...                                        *
    // **********************************************
    //
    // Unbounded Ports  will be reported as errors and marked red
    // Cannot invalidate DAG property, connecting ports must obey this
    // Types must unify when connecting ports else report error
    // If Node input is ordered, order is top down
    // Else mark the ports differently
    //
    // Lambdas can acces consants and outer parameters of an op. i.e. they capture the env
    //
    // When selecting a subgraph A from the main graph to create a new Node/operation NewOp
    // You can delete a subgraph B in A to make the NewOp node accept lambdas.
    // In this NewOp creation, compiler will figure out the constants used in the rectangle and
    // will create the constants in NewNode accordingly
    //
    //
    // Basic Ops:
    //    Sink<T>  Signature { (Many(T)) -> () }
    //    Source<T> Signature { () -> Strean<T> }
    //    Const<T> Signature  { (T:Const) -> T
    //                        }
    //
    //   toStream Signatures { T -> Stream<T> }
    //
    //    Add  Signatures  { (Many(Int)) -> Int
    //                       (Many Float) -> Float
    //                       (Many(Vec<T,n) -> Vec<T,n>)
    //                       (Many(Covec<T,n>) -> Covec<T,n>)
    //                       (Many Matrix<T,N,M>) -> Matrix<T,N,M>
    //                       (Many(Vec(T,n)) , Many(Int)) -> Vec<T,n>
    //                       (Many(CoVec(T,n)) , Many(Int)) -> Vec<T,n>
    //                       (Many(Matric(T,n,m)) , Many(Int)) -> Vec<T,n,m>
    //
    //                       (Many(Vec(T,n)) , Many(Mat,n,m)) -> Mat<T,n,m>
    //                       (Many(CoVec(T,n)) , Many(Mat,m,n)) -> Mat<T,m,n>
    //
    //                       (.. Same with Stream<Int|Vec> .... )
    //
    //                     }
    //    Mul  Signatures  { }
    //    Max  Signatures  { }
    //    Min  Signatures  { }
    //    Sub  Signatures  { }
    //    Div  Signatures  { } (similar to add , consider the true matrix math though)
    //
    //    Norm Signatures  { vec -> float, covec -> float , mat -> float}
    //    mean / variance / etc.
    //
    //
    //    SlidingBuffer Signatures  { stream -> vec }
    //    WindowBuffer  Signatures  { stream -> vec  }
    //    Zip    Signatures  { }
    //
    //    Map    Signatures  { }
    //    Filter Signatures  { }
    //    Fold   Signatures  { }
    //    UnFold Signature   { }
    //
    //    MapN / FilterN / FoldN / UnFoldN etc.
    //
    //
    //
    //
    //
    //
    //
}

struct NodeInst {
    id: Idx<NodeInstIdx>,
    name: Idx<NodeNameIdx>,
    signature: SetIdx<SignatureIdx>,
    consts: VecIdx<ConstIdx>,
    inputs: VecIdx<InputPortIdx>,
    outputs: VecIdx<OutputPortIdx>,
    fun: Idx<FnIdx>,
    lambda: Option<LambdaParams>,
}

struct Pool {
    type_var: usize,
    aritied_types: Vec<AritiedType>,
    types: Vec<Type>,

    nodes: Vec<Node>,
    names: Vec<String>,
    signature: Vec<Signature>,
    fn_pool: Vec<Box<dyn FnMut(&mut Env, VecIdx<ValueIdx>, VecIdx<ConstIdx>) -> Vec<ValueIdx>>>,
    inputs: Vec<InputPort>,
    outputs: Vec<OutputPort>,
    instants: Vec<NodeInst>,

    const_strings: Vec<String>,
    const_datetime: Vec<chrono::DateTime<chrono::Utc>>,
    consts: Vec<Const>,

    value_strings: Vec<String>,
    value_datetime: Vec<chrono::DateTime<chrono::Utc>>,
    values: Vec<Value>,
    streams: Vec<Box<dyn Iterator<Item = Value>>>,
}

fn add_x<T: Copy + Clone + Debug + PartialEq>(s: &mut Vec<T>, arity_type: T) -> (Idx<T>, T) {
    s.push(arity_type);
    let idx = Idx(s.len() - 1, PhantomData);
    (idx, *s.last().unwrap())
}
fn get_x<T: Copy + Clone + Debug + PartialEq>(s: &Vec<T>, idx: Idx<T>) -> Option<T> {
    s.get(idx.0).copied()
}
fn get_mut_x<T: Copy + Clone + Debug + PartialEq>(s: &mut Vec<T>, idx: Idx<T>) -> Option<&mut T> {
    s.get_mut(idx.0)
}
fn get_ref<T: Copy + Clone + Debug + PartialEq>(s: &Vec<T>, idx: Idx<T>) -> Option<&T> {
    s.get(idx.0)
}
fn get_unchecked_x<T: Copy + Clone + Debug + PartialEq>(s: &Vec<T>, idx: Idx<T>) -> T {
    unsafe { s.get_unchecked(idx.0).clone() }
}
fn get_unchecked_mut_x<T: Copy + Clone + Debug + PartialEq>(s: &mut Vec<T>, idx: Idx<T>) -> &mut T {
    unsafe { s.get_unchecked_mut(idx.0) }
}
fn get_unchecked_ref_x<T: Copy + Clone + Debug + PartialEq>(s: &Vec<T>, idx: Idx<T>) -> &T {
    unsafe { s.get_unchecked(idx.0) }
}

fn main() {
    println!("Hello, world!");
}
