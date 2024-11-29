import { Button } from "@mui/material";

const MenuButton = (props) => {
  return (
    <Button variant="contained" {...props}>
      {props.children}
    </Button>
  );
};

export default MenuButton;
