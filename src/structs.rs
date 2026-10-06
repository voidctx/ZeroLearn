pub type NeuronId = u32;

pub struct Neuron {
    pub id: NeuronId,
    pub seed: u64,
    pub outbound: Vec<NeuronId>,
}

pub struct Brain {
    pub neurons: Vec<Neuron>,
    pub temp: f32,
    pub lr: f32,
    pub input: Vec<NeuronId>,
    pub output: Vec<NeuronId>,
}

pub struct Coeffs {
    pub a1: f32,
    pub a2: f32,
    pub a3: f32,
    pub a4: f32,
    pub a5: f32,
    pub a6: f32,
    pub a7: f32,
    pub a8: f32,
}