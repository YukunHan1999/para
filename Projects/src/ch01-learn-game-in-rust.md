# 1. joy and skog game learn

## 1.1 依赖关键字梳理

- use std::sync::Arc;  标准库原子引用
- winit 跨平台窗口管理与事件循环
- wgpu 基于WebGPU标准的现代图形API库
- gilrs 游戏手柄输入库
- platform 针对特定平台的时间库



- crate::core::core 引擎的核心上下文或启动入口
- crate::scene::SceneStack 场景栈,用来管理游戏状态切换
- crate::scene::{Layer, Scene, SceneAction, SceneResources}: 具体场景定义，涂层管理，场景间传递的动作和资源

- crate::assets::AssetManager, Assets: 全局资产管理器，从磁盘或网络加载资源
- crate::assets::audio::SoundId: 音频资源的唯一标识符
- crate::textures::{Atlas, AtlasId, TextureResorceId}: 纹理集，多张小图合并成一张大图以优化渲染性能
- crate::materials::MaterialId: 材质Id(决定为物体表面如何与光照和着色器交互)
- crate::store::{Map, Set, PushList, SecondaryMap, Slab, Sparse} 引擎自定义的高性能底层数据结构

## 1.2 GPU渲染




```mermaid
%%{init: {'theme': 'dark', 'themeVariables': { 'lineColor': '#ffffff', 'edgeLabelBackground': '#000000', 'actorTextColor': '#ffffff' }}}%%
flowchart TB

    %% ==========================================
    %% 领地一：硬件与上下文初始化 (高亮亮橙边框)
    %% ==========================================
    subgraph Initialization ["@ 1. 硬件与上下文初始化"]
        Instance["GPU实例<br>(Instance)"]
        Adapter["适配器<br>(Adapter)"]
        Device["逻辑设备<br>(Device)"]
        Surface["展示平面<br>(Surface)"]
        
        Instance --> Adapter
        Instance --> Surface
        Adapter --> Device
    end

    %% ==========================================
    %% 领地二：交换链与画布准备 (高亮翠绿边框)
    %% ==========================================
    subgraph CanvasPrep ["@ 2. 渲染目标与画布准备"]
        SurfaceTexture["纹理资源容器<br>(SurfaceTexture)"]
        TextureView["纹理视图<br>(TextureView)"]
        ColorAttachment["颜色附件<br>(ColorAttachment)"]
        
        SurfaceTexture --> TextureView
        TextureView -->|"定义画在哪里"| ColorAttachment
    end

    %% ==========================================
    %% 领地三：命令编码与执行 (高亮酷蓝边框)
    %% ==========================================
    subgraph Execution ["@ 3. 动态命令流水线"]
        Pipeline["[renderer.rs #L20]<br>渲染管线 (RenderPipeline)"]
        Buffers["[renderer.rs #L35]<br>资源缓冲 (Buffers)"]
        Encoder["[renderer.rs #L50]<br>命令编码器 (Encoder)"]
        RenderPass["渲染通道<br>(RenderPass)"]
        CommandBuffer["CommandBuffer"]
        Queue["命令队列<br>(Queue)"]
        
        Encoder --> RenderPass
        Pipeline -->|"怎么画"| RenderPass
        Buffers -->|"画什么"| RenderPass
        
        Encoder -->|"finish"| CommandBuffer
        CommandBuffer -->|"submit"| Queue
    end

    %% ==========================================
    %% 跨子图的连接（全高亮白线）
    %% ==========================================
    Device --> Pipeline
    Device --> Encoder
    Device --> Queue
    
    Surface --> SurfaceTexture
    ColorAttachment -->|"画布底板"| RenderPass
    
    SurfaceTexture -->|"present()"| Screen[("屏幕显示")]

    %% ==========================================
    %% 强制重写子图样式（高对比度，绝不发愁）
    %% ==========================================
    style Initialization fill:#1e140a,stroke:#ff9800,stroke-width:3px,color:#ffb74d
    style CanvasPrep fill:#0a1e0a,stroke:#4caf50,stroke-width:3px,color:#81c784
    style Execution fill:#0a1428,stroke:#2196f3,stroke-width:3px,color:#64b5f6

    %% ==========================================
    %% 强制重写内部所有节点样式（黑底 + 高亮粗边框 + 纯白字）
    %% ==========================================
    classDef nodeStyle fill:#111111,stroke:#ffffff,stroke-width:2.5px,color:#ffffff;
    classDef screenStyle fill:#ffffff,stroke:#ffffff,stroke-width:3px,color:#000000;
    
    class Instance,Adapter,Device,Surface,SurfaceTexture,TextureView,ColorAttachment,Pipeline,Buffers,Encoder,RenderPass,CommandBuffer,Queue nodeStyle;
    class Screen screenStyle;
```
