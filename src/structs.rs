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