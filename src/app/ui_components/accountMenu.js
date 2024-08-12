import Avatar from '@mui/material/Avatar';
import Box from '@mui/material/Box';
import IconButton from '@mui/material/IconButton';
import Tooltip from '@mui/material/Tooltip';
import { invoke } from "@tauri-apps/api/core"
import * as React from 'react';

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
    this.setState({ initials:nameSplit[0].charAt(0) + nameSplit[0].charAt(0) });
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
