use crate::structure::{BlockState, Coordinates, Region};

const BIT_TO_LONG_SHIFT: u8 = 6; //log2(64)

impl Region {
    /// Retrieves the block state at the specified position within the region.
    ///
    /// This function takes a position which can be converted into `Coordinates` and returns a reference to the `BlockState`
    /// at that position within the region.
    ///
    /// # Examples
    ///
    /// ```
    /// let region = ...; // Load or create a Region
    /// let block = region.get_block((3, 5, 7));
    /// println!("Block state at position (3, 5, 7): {:?}", block);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `position` - A value that can be converted into `Coordinates` representing the position of the block within the region
    ///
    /// # Returns
    ///
    /// * A reference to the `BlockState` at the specified position within the region
    pub fn get_block(&self, position: impl Into<Coordinates>) -> &BlockState {
        let position = position.into();
        let index = self.get_3d_index(position);

        let required_bits = Self::calc_required_bits(&self.block_state_palette);

        let mask = (1 << required_bits) - 1;

        let palette_index = self.get_palette_index(index, required_bits, mask);

        &self.block_state_palette[palette_index as usize]
    }

    /// Retrieves the block state as mutable at the specified position within the region.
    ///
    /// This function takes a position which can be converted into `Coordinates` and returns a reference to the `BlockState`
    /// at that position within the region.
    ///
    /// # Examples
    ///
    /// ```
    /// let region = ...; // Load or create a Region
    /// let block = region.get_block_mut((3, 5, 7));
    /// block.set_name("stone");
    /// println!("Block state at position (3, 5, 7): {:?}", block);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `position` - A value that can be converted into `Coordinates` representing the position of the block within the region
    ///
    /// # Returns
    ///
    /// * A mutable reference to the `BlockState` at the specified position within the region
    pub fn get_block_mut(&mut self, position: impl Into<Coordinates>) -> &mut BlockState {
        let position = position.into();
        let index = self.get_3d_index(position);

        let required_bits = Self::calc_required_bits(&self.block_state_palette);

        let mask = (1 << required_bits) - 1;

        let palette_index = self.get_palette_index(index, required_bits, mask);

        &mut self.block_state_palette[palette_index as usize]
    }

    /// Sets the block state at the specified position within the region.
    ///
    /// This function takes a position which can be converted into `Coordinates` and a `BlockState` to be set at that position
    /// within the region.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut region = ...; // Load or create a mutable Region
    /// let new_block = BlockState { name: "minecraft:stone".to_string(), properties: HashMap::new() };
    /// region.set_block((3, 5, 7), new_block);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `position` - A value that can be converted into `Coordinates` representing the position of the block within the region
    /// * `block` - The `BlockState` to be set at the specified position within the region
    pub fn set_block(&mut self, position: impl Into<Coordinates>, block: BlockState) {
        let position = position.into();
        let index = self.get_3d_index(position);

        let mut bits = Self::calc_required_bits(&self.block_state_palette);

        let mut mask = (1 << bits) - 1;

        let palette_index = self
            .block_state_palette
            .iter()
            .position(|b| *b == block)
            .unwrap_or_else(|| {
                let index = self.block_state_palette.len();

                // minimum size is 2 bits
                if index.is_power_of_two() && index >= 4 {
                    let new_bits = bits + 1;
                    let new_mask = (1 << new_bits) - 1;

                    self.resize_block_states(bits, mask, new_bits, new_mask);

                    bits = new_bits;
                    mask = new_mask;
                }

                self.block_state_palette.push(block);
                index
            });

        Self::set_block_index(
            &mut self.block_states,
            index,
            palette_index as u32,
            bits,
            mask,
        );
    }

    /// Calculates the number of bits required to represent block states in the palette.
    ///
    /// Given a palette of `BlockState`s, this function calculates the minimum number of bits needed to represent
    /// each block state in the palette, with a minimum of 2 bits.
    ///
    /// # Examples
    ///
    /// ```
    /// let palette = vec![block_state1, block_state2, block_state3];
    /// let required_bits = calc_required_bits(&palette);
    /// println!("Required bits for the palette: {}", required_bits);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `palette` - A reference to a `Vec<BlockState>` representing the palette of block states
    ///
    /// # Returns
    ///
    /// * The minimum number of bits required to represent the block states in the palette, as a `u64`
    fn calc_required_bits(palette: &Vec<BlockState>) -> u64 {
        palette.len().next_power_of_two().trailing_zeros().max(2) as u64
    }

    /// Calculates the linear index of a block within the region given its 3D coordinates.
    ///
    /// This function takes a `Coordinates` value representing the position of a block within the region and returns
    /// a linear index that can be used to access the block's state in the region's data structures.
    ///
    /// # Examples
    ///
    /// ```
    /// let region = ...; // Load or create a Region
    /// let coords = Coordinates { x: 1, y: 2, z: 3 };
    /// let index = region.get_3d_index(coords);
    /// println!("Linear index for position (1, 2, 3): {}", index);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `coords` - A `Coordinates` value representing the position of a block within the region
    ///
    /// # Returns
    ///
    /// * The linear index of the block with the given 3D coordinates within the region, as a `u64`
    fn get_3d_index(&self, coords: Coordinates) -> u64 {
        // check that the coordinates are withoin the bounds of the region
        assert!(coords.x >= 0 && coords.x < self.size.x.abs());
        assert!(coords.y >= 0 && coords.y < self.size.y.abs());
        assert!(coords.z >= 0 && coords.z < self.size.z.abs());

        // convert the coordinates to unsigned integers
        let x = coords.x as u64;
        let y = coords.y as u64;
        let z = coords.z as u64;

        // calculate the linear index
        let size_x = self.size.x.abs() as u64;
        let size_layer = size_x * self.size.z.abs() as u64;

        y * size_layer + z * size_x + x
    }

    /// Returns the palette index for a given block_index, required_bits and bitmask.
    ///
    /// # Arguments
    ///
    /// * `block_index` - The block index to retrieve the palette index for
    /// * `required_bits` - The number of bits used for each block state in the block states array
    /// * `bitmask` - The bitmask used to isolate the desired bits from the block states array
    ///
    /// # Returns
    ///
    /// The palette index corresponding to the given block index as a `u32`
    fn get_palette_index(&self, block_index: u64, required_bits: u64, bitmask: u32) -> u32 {
        let bit_position = block_index * required_bits;
        let word_index = (bit_position >> BIT_TO_LONG_SHIFT) as usize;

        let end_word_index =
            (((block_index + 1) * required_bits - 1) >> BIT_TO_LONG_SHIFT) as usize;
        let index_in_word = (bit_position ^ ((word_index as u64) << BIT_TO_LONG_SHIFT)) as u8;

        if word_index == end_word_index {
            (self.block_states[word_index] >> index_in_word) as u32 & bitmask
        } else {
            let first_bits = 64 - index_in_word;
            ((self.block_states[word_index] >> index_in_word) as u32 & bitmask)
                | ((self.block_states[end_word_index] << first_bits) as u32 & bitmask)
        }
    }

    /// Sets the palette index for a given block index.
    ///
    /// # Arguments
    ///
    /// * `block_states` - The block states array to set the palette index in
    /// * `block_index` - The block index to set the palette index for
    /// * `value` - The palette index to set
    /// * `required_bits` - The number of bits used for each block state in the block states array
    /// * `bitmask` - The bitmask used to isolate the desired bits from the block states array
    fn set_block_index(
        block_states: &mut [i64],
        block_index: u64,
        value: u32,
        required_bits: u64,
        bitmask: u32,
    ) {
        let bit_position = block_index * required_bits;
        let word_index = (bit_position >> BIT_TO_LONG_SHIFT) as usize;

        let end_word_index =
            (((block_index + 1) * required_bits - 1) >> BIT_TO_LONG_SHIFT) as usize;

        let index_in_word = (bit_position ^ ((word_index as u64) << BIT_TO_LONG_SHIFT)) as u8;

        block_states[word_index] = (block_states[word_index]
            & !((bitmask as i64) << index_in_word))
            | (((value & bitmask) as i64) << index_in_word);

        if word_index != end_word_index {
            let bits_written = 64 - index_in_word;
            let bits_to_write = required_bits as u8 - bits_written;

            block_states[end_word_index] = (block_states[end_word_index]
                & !((1 << bits_to_write) - 1))
                | ((value & bitmask) >> bits_written) as i64;
        }
    }

    /// Resizes the block states array to accommodate a new number of bits per block, updating the palette indices accordingly.
    /// This function adjusts the internal storage of block palette indices when the number of bits required to represent the indices changes (e.g., due to a change in the palette size).
    ///
    /// # Arguments
    ///
    /// * `old_required_bits` - The number of bits used for each block state in the block states array before the resize
    /// * `old_bitmask` - The bitmask used to isolate the desired bits from the block states array before the resize
    /// * `new_required_bits` - The number of bits used for each block state in the block states array after the resize
    /// * `new_bitmask` - The bitmask used to isolate the desired bits from the block states array after the resize
    fn resize_block_states(
        &mut self,
        old_required_bits: u64,
        old_bitmask: u32,
        new_required_bits: u64,
        new_bitmask: u32,
    ) {
        let volume = self.size.x.abs() as u64 * self.size.y.abs() as u64 * self.size.z.abs() as u64;
        let required_bits_per_block = (volume * new_required_bits + 63) >> BIT_TO_LONG_SHIFT; // rounding up

        let mut new_blockstates: Vec<i64> = vec![0; required_bits_per_block as usize];

        for i in 0..volume {
            let old_palette_index = self.get_palette_index(i, old_required_bits, old_bitmask);

            Self::set_block_index(
                &mut new_blockstates,
                i,
                old_palette_index,
                new_required_bits,
                new_bitmask,
            );
        }

        self.block_states = new_blockstates;
    }
}
