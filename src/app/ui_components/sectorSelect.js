import * as React from 'react';
import Box from '@mui/material/Box';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';
import { OutlinedInput } from '@mui/material';
import { get_all_sectors } from '../funcs/stock_api';

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
            sector: [],
            sectors: []
        }
        this.handleChange = this.handleChange.bind(this);
    }

    handleChange(event) {
        const sector = event.target.value;
        this.setState({ sector })
        this.props.set_sector(sector)
    }

    componentDidMount() {
        get_all_sectors().then(sectors => { 
            const default_sector = "Default";
            sectors.unshift(default_sector);
            this.setState({ sectors: sectors, sector: default_sector }) 
        })
    }

    render() {
        const { sector, sectors } = this.state;
        return (
            <div>
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
                            {sectors.map((sector) => (
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
            </div>
        )
    };
}
