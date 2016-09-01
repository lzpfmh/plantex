initSidebarItems({"constant":[["CHUNK_SIZE","How many hex pillars a chunk is long. So the number of hex pillars in a chunk is `CHUNK_SIZE`²."],["HEX_INNER_RADIUS","Inner radius of the hexagons"],["HEX_OUTER_RADIUS","Outer radius of the hexagons (from center to corner)"],["PILLAR_STEP_HEIGHT","The height of the hex pillars is discretized. So instead of saving a `f32` to represent the height, we have fixed steps of heights and we will just save a `u16` to represent the height."]],"mod":[["chunk",""],["ground",""]],"struct":[["ChunkIndex","A new-type to index chunks. This is different from the `PillarIndex` type which always represents a pillar position. So two different `PillarIndex`es could refer to two pillars in the same chunk, while two different `ChunkIndex`es always refer to two different chunks."],["FallbackProvider","A fallback provider that holds two chunk providers with one being primary and one fallback. If the chunk load from the primary fails the fallback is being called."],["HeightType","Represents a discretized height."],["HexPillar","Represents one pillar of hexgonal shape in the game world."],["NullProvider","A dummy provider that always fails to provide a chunk."],["PillarIndex","A new-type to index Pillars. Always represents a specific Pillar in absolute world coordinates."],["PillarSection","Represents one section of a hex pillar."],["Prop","A prop in a hex pillar"],["World","Represents a whole game world consisting of multiple `Chunk`s."]],"trait":[["ChunkProvider","A type that can load a game world, specifically single chunks of it. This could mean loading a saved world from a file, generating a world procedurally or loading a world from a server."]],"type":[["PillarIndexComponent","This type is used to index into one dimension of the world. Thus we can \"only\" index `(PillarIndexComponent::max_value() - PillarIndexComponent::min_value())`² many hex pillars."]]});