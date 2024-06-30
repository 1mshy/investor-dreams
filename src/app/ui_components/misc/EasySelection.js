import { OutlinedInput } from '@mui/material';
import Box from '@mui/material/Box';
import FormControl from '@mui/material/FormControl';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import Select from '@mui/material/Select';
import React, { Component } from 'react';

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

export default class EasySelection extends Component {
    /**
     * props.label - label for the selection
     * props.content - object with the display item as key
     * and function as the value
     * The function will be run when the user selects the item from the list
     * ex: {"firs tthing": ()=> {}, "second thing": ()=>{}}
     */
    constructor(props) {
        super(props);
        this.state = {
            current_item: "Weight"
        }
        
        this.handle_change = this.handle_change.bind(this);
    }

    handle_change(event) {
        const item = event.target.value;
        this.props.content[item]();
        this.setState({ current_item: item });
    }
    render() {
        const { label, content } = this.props;
        const { current_item } = this.state;
        return <Box sx={{ minWidth: 120 }}>
            <FormControl sx={{ m: 1, width: 300 }}>
                <InputLabel id="selection">{label}</InputLabel>
                <Select
                    labelId="selection"
                    id="selection"
                    label={label}
                    value={current_item}
                    onChange={this.handle_change}
                    input={<OutlinedInput label={label} />}
                    MenuProps={MenuProps}
                >
                    {Object.keys(content).map((item) => (
                        <MenuItem
                            key={`${item}_`}
                            value={item}
                        >
                            {item}
                        </MenuItem>
                    ))}
                </Select>
            </FormControl>
        </Box>
    }
}