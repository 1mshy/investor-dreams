import * as React from 'react';
import Box from '@mui/material/Box';
import Avatar from '@mui/material/Avatar';
import Menu from '@mui/material/Menu';
import MenuItem from '@mui/material/MenuItem';
import ListItemIcon from '@mui/material/ListItemIcon';
import Divider from '@mui/material/Divider';
import IconButton from '@mui/material/IconButton';
import Typography from '@mui/material/Typography';
import Tooltip from '@mui/material/Tooltip';
import PersonAdd from '@mui/icons-material/PersonAdd';
import Settings from '@mui/icons-material/Settings';
import Logout from '@mui/icons-material/Logout';
import { invoke } from '@tauri-apps/api';

class AccountMenu extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      initials: ''
    };
  }
  /**
   * @description Fetches the username from the backend and sets it to the initials state
   * @description this is run **ONCE** when the component is mounted (fully rendered)
   */
  async componentDidMount() {
    // Got the method from Home.js if there's a better way please let me know... I wanna learn :)
    // Defined in case of errors etc. - wouldnt brick the program
    const username = await invoke("get_username")
    const nameSplit = username.split(" ");
    this.setState({ initials:nameSplit[0].charAt(0) + nameSplit[1].charAt(0) });
  }

  render() {
    const { anchorEl, initials } = this.state;
    const open = Boolean(anchorEl);

    return (
      <>
        <Box sx={{ display: 'flex', alignItems: 'center', textAlign: 'center', justifyContent: "flex-end", marginLeft: "auto", marginRight: "2%"}}>
          <Tooltip title="Account settings">
            <IconButton
              // onClick={this.handleClick}
              size="small"
              sx={{ ml: 2 }}
              aria-controls={open ? 'account-menu' : undefined}
              aria-haspopup="true"
              aria-expanded={open ? 'true' : undefined}
            >
              <Avatar sx={{ width: 32, height: 32 }}>
                {/* Initial should be displayed here */}
                {initials}
              </Avatar>
            </IconButton>
          </Tooltip>
        </Box>
      </>
    );
  }
}

export default AccountMenu;
