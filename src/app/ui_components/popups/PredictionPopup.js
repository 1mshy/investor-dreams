import React, { Component } from "react";
import { Backdrop, Dialog } from '@mui/material';
import { Transition } from "@/app/funcs/themes";
import { SoftPaper } from "@/app/mui/theme";

export default class PredictionPopup extends Component {
    constructor(props) {
        super(props);
        this.state = {
            open: false,
        }
        this.clicked = this.clicked.bind(this);

    }

    clicked() {
        this.setState({ open: !this.state.open });
    }

    render() {
        const { open } = this.state;
        return <>
            <Backdrop open={open} onClick={this.clicked} invisible={true} style={{ width: "100%", maxWidth: "100%" }}>
                <Dialog
                    open={open}
                    aria-labelledby="responsive-dialog-title"
                    TransitionComponent={Transition}
                    maxWidth='lg'
                    fullWidth
                    PaperComponent={SoftPaper}
                    PaperProps={{
                        sx: {
                            width: "100%",
                            maxWidth: "90%",
                            height: "100%",
                            maxHeight: "80%",
                        }
                    }}
                >

                </Dialog>
            </Backdrop>
            <div onClick={() => this.setState({ open: true })}>
                {this.props.children}
            </div>
        </>
    }
}