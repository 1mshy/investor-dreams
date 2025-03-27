import * as React from 'react';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';
import { ListSubheader, OutlinedInput } from '@mui/material';
import { get_all_sectors } from '@/app/networking/stock_api';
import Box from '@mui/material/Box';

const ITEM_HEIGHT = 48;
const ITEM_PADDING_TOP = 8;
const MenuProps = {
    disableScrollLock: true,
    marginThreshold: null,
    PaperProps: {
        style: {
            maxHeight: ITEM_HEIGHT * 4.5 + ITEM_PADDING_TOP,
            width: 250,
        },
    },
};

export default class SectorSelect extends React.Component {
    constructor(props) {
        super(props)
        this.state = {
            sector: "",
            generated_sectors: [],
        }
        this.handleChange = this.handleChange.bind(this);
    }

    handleChange(event) {
        const sector = event.target.value;
        this.setState({ sector })
        this.props.set_sector(sector)
    }

    async componentDidMount() {
        const sectors = await get_all_sectors()
        this.setState({ generated_sectors: sectors })
    }
    /**
     * Updates the sector state if the default sector is set
     * Need to wait for the parent component to set the default sector
     * @returns {Promise<void>}
     */
    async componentDidUpdate() {
        if (!this.state.sector && this.props.default_sector) {
            this.setState({ sector: this.props.default_sector })
        }
    }

    render() {
        const { sector, generated_sectors } = this.state;
        const {custom_sectors} = this.props;
        return (
            <Box sx={{ minWidth: 120 }}>
                <FormControl sx={{ m: 1, width: 300 }}>
                    <InputLabel id="sectorSelectLabel">Sector</InputLabel>
                    <Select
                        labelId="sectorSelectLabel"
                        id="sectorSelect"
                        label="Sector"
                        value={sector}
                        onChange={this.handleChange}
                        input={<OutlinedInput label="Sector" />}
                        MenuProps={MenuProps}
                    >
                        <ListSubheader>Custom</ListSubheader>
                        {custom_sectors.map((sector) => (
                            <MenuItem
                                key={sector}
                                value={sector}
                            >
                                {sector}
                            </MenuItem>
                        ))}
                        <ListSubheader>Default</ListSubheader>
                        {generated_sectors.map((sector) => (
                            <MenuItem
                                key={sector}
                                value={sector}
                            >
                                {sector}
                            </MenuItem>
                        ))}
                    </Select>
                </FormControl>
            </Box>
        )
    };
}
