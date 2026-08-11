#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    texcoord: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, -0.5, -0.5],
        texcoord: [0.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5, -0.5],
        texcoord: [1.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
        texcoord: [1.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5, -0.5],
        texcoord: [0.0, 1.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.5],
        texcoord: [0.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.5],
        texcoord: [1.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
        texcoord: [1.0, 1.0],
    },
    Vertex {
        position: [-0.5, 0.5, 0.5],
        texcoord: [0.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5, -0.5],
        texcoord: [0.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
        texcoord: [1.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
        texcoord: [1.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.5],
        texcoord: [0.0, 1.0],
    },
    Vertex {
        position: [-0.5, -0.5, -0.5],
        texcoord: [0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.5],
        texcoord: [1.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5, 0.5],
        texcoord: [1.0, 1.0],
    },
    Vertex {
        position: [-0.5, 0.5, -0.5],
        texcoord: [0.0, 1.0],
    },
    Vertex {
        position: [-0.5, 0.5, -0.5],
        texcoord: [0.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5, 0.5],
        texcoord: [1.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
        texcoord: [1.0, 1.0],
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
        texcoord: [0.0, 1.0],
    },
    Vertex {
        position: [-0.5, -0.5, -0.5],
        texcoord: [0.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, -0.5],
        texcoord: [1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.5],
        texcoord: [1.0, 1.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.5],
        texcoord: [0.0, 1.0],
    },
];

#[rustfmt::skip]
pub const INDICES: &[u16] = &[
    0, 1, 2, 0, 2, 3,
    4, 5, 6, 4, 6, 7,
    8, 9, 10, 8, 10, 11,
    12, 13, 14, 12, 14, 15,
    16, 17, 18, 16, 18, 19,
    20, 21, 22, 20, 22, 23,
];
