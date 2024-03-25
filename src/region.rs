use crate::{
    block::BlockStatePattern,
    structure::{BlockState, Coordinates, Region},
};

const BIT_TO_LONG_SHIFT: u8 = 6; //log2(64)

impl Region {
    pub fn get_block(&self, position: impl Into<Coordinates>) -> &BlockState {
        let position = position.into();
        let block_index = self.get_3d_index(position);

        let required_bits = Self::calc_required_bits(&self.block_state_palette);

        let bitmask = (1 << required_bits) - 1;

        let palette_index = self.get_palette_index(block_index, required_bits, bitmask);

        &self.block_state_palette[palette_index as usize]
    }

    pub fn get_block_mut(&mut self, position: impl Into<Coordinates>) -> &mut BlockState {
        let position = position.into();
        let index = self.get_3d_index(position);

        let required_bits = Self::calc_required_bits(&self.block_state_palette);

        let mask = (1 << required_bits) - 1;

        let palette_index = self.get_palette_index(index, required_bits, mask);

        &mut self.block_state_palette[palette_index as usize]
    }

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

    pub fn find_block_positions(
        &self,
        block_state: &impl BlockStatePattern,
    ) -> impl Iterator<Item = Coordinates> {
        let mut matching = Vec::new();

        for y in 0..self.size.y.abs() {
            for z in 0..self.size.z.abs() {
                for x in 0..self.size.x.abs() {
                    let coords = Coordinates::from((x, y, z));

                    let block = self.get_block(coords);

                    if block_state.matches(block) {
                        matching.push(coords);
                    }
                }
            }
        }

        matching.into_iter()
    }

    pub(crate) fn calc_required_bits(palette: &Vec<BlockState>) -> u64 {
        palette.len().next_power_of_two().trailing_zeros().max(2) as u64
    }

    pub(crate) fn get_3d_index(&self, coords: impl Into<Coordinates>) -> u64 {
        let coords = coords.into();

        // check that the coordinates are withoin the bounds of the region
        assert!(coords.x >= 0 && coords.x < self.size.x.abs());
        assert!(coords.y >= 0 && coords.y < self.size.y.abs());
        assert!(coords.z >= 0 && coords.z < self.size.z.abs());

        // convert the coordinates to unsigned integers
        let x = coords.x as u64;
        let y = coords.y as u64;
        let z = coords.z as u64;

        // calculate the linear index
        let size_x = self.size.x.unsigned_abs() as u64;
        let size_layer = size_x * self.size.z.unsigned_abs() as u64;

        y * size_layer + z * size_x + x
    }

    pub(crate) fn get_palette_index(
        &self,
        block_index: u64,
        required_bits: u64,
        bitmask: u32,
    ) -> u32 {
        let bit_index = block_index * required_bits;
        let word_index = (bit_index >> BIT_TO_LONG_SHIFT) as usize;
        let end_word_index =
            (((block_index + 1) * required_bits - 1) >> BIT_TO_LONG_SHIFT) as usize;
        let index_in_word = (bit_index ^ ((word_index as u64) << BIT_TO_LONG_SHIFT)) as u8;

        if word_index == end_word_index {
            (self.block_states[word_index] >> index_in_word) as u32 & bitmask
        } else {
            let first_bits = 64 - index_in_word; // 2

            ((self.block_states[word_index] as u64 >> index_in_word) as u32 & bitmask)
                | ((self.block_states[end_word_index] << first_bits) as u32 & bitmask)
        }
    }

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

    fn resize_block_states(
        &mut self,
        old_required_bits: u64,
        old_bitmask: u32,
        new_required_bits: u64,
        new_bitmask: u32,
    ) {
        let volume = self.calc_volume();
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

    fn calc_volume(&self) -> u64 {
        self.size.x.unsigned_abs() as u64
            * self.size.y.unsigned_abs() as u64
            * self.size.z.unsigned_abs() as u64
    }
}

#[cfg(test)]
mod tests {
    use crate::{resource_location::ResourceLocation, structure::LitematicaFile};
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn get_3d_index() {
        let litematic = LitematicaFile::read("test.litematic").unwrap();
        let region = litematic.get_region("test").unwrap(); // region size: 31x9x29

        assert_eq!(region.get_3d_index((0, 0, 0)), 0);
        assert_eq!(region.get_3d_index((30, 0, 0)), 30);
        assert_eq!(region.get_3d_index((0, 8, 0)), 31 * 29 * 8);
    }

    #[test]
    fn get_palette_index() {
        let litematic = LitematicaFile::read("test.litematic").unwrap();
        let region = litematic.get_region("test").unwrap(); // region size: 31x9x29

        let _palette_len = region.block_state_palette.len(); // 25

        let required_bits = Region::calc_required_bits(&region.block_state_palette); // 5
        let bitmask = (1 << required_bits) - 1; // 31

        let block_index = region.get_3d_index((0, 2, 0)); // 31 * 29 * 2 = 1.798
        let palette_index = region.get_palette_index(block_index, required_bits, bitmask);
        assert_eq!(palette_index, 0);
        assert_eq!(
            region.block_state_palette[palette_index as usize].name,
            ResourceLocation::minecraft("air")
        );

        let block_index = region.get_3d_index((2, 4, 2)); // 3660
        let palette_index = region.get_palette_index(block_index, required_bits, bitmask);

        assert_eq!(palette_index, 24);
        assert_eq!(
            region.block_state_palette[palette_index as usize].name,
            ResourceLocation::minecraft("powered_rail")
        );
        assert_eq!(
            region.block_state_palette[palette_index as usize].properties,
            HashMap::from([
                ("shape".to_string(), "north_south".to_string()),
                ("powered".to_string(), "true".to_string()),
                ("waterlogged".to_string(), "false".to_string())
            ])
        );
    }

    #[test]
    fn idk_how_this_works() {
        let litematic = LitematicaFile::read("test.litematic").unwrap();

        println!("{:#?}", litematic.get_region("test"));
    }
}
