import styled from "@emotion/styled";
import { SearchOutlined } from "@mui/icons-material";
import { InputAdornment, TextField } from "@mui/material";

const StyledTextField = styled(TextField)({
    '& .MuiInputBase-root': {
        borderRadius: '20px',
        paddingLeft: '10px',
        paddingRight: '10px',
        border: '1px solid transparent',
        transition: 'border-color 0.2s',
        cursor: 'default',
        height: '100%',
        minHeight: '40px',
        minWidth: 'fit-content',
        width: '10rem',
        '& input': {
            cursor: 'text',
        },
    },
    '& .MuiInputBase-root:hover': {
        borderColor: '#909090',
    },
    '& .MuiInputBase-root.Mui-focused': {
        borderColor: '#909090',
        borderWidth: '0.1rem',
    },
    '& .MuiInput-underline:before, & .MuiInput-underline:after': {
        display: 'none',
    },
});

export const SearchBarTop = (params) => {
    return (
        <StyledTextField
            name="text"
            autoComplete="off"
            type="text"
            variant="standard"
            InputProps={{
                disableUnderline: true,
                startAdornment: (
                    <InputAdornment position="start">
                        <SearchOutlined />
                    </InputAdornment>
                ),
            }}
            {...params}
        />
    );
};
