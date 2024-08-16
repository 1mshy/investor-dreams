import { Backdrop, DialogContent, DialogContentText } from "@mui/material";
import React, { Component } from "react";

import DialogTitle from '@mui/material/DialogTitle';
import Dialog from '@mui/material/Dialog';

export default class GeneralPopup extends Component {
    constructor(props) {
        super(props);
        this.state = { open: !!props.open, };
        this.handleClose = this.handleClose.bind(this);
    }

    handleClose(event, reason) {
        if (reason && reason === "backdropClick") {
            this.setState({ open: false });
        }
    }

    render() {
        const { open } = this.state;
        return (
            <>
                <Backdrop open={open} invisible={true}>
                    <Dialog
                        open={open}
                        onClose={this.handleClose}
                        aria-labelledby="responsive-dialog-title"
                        maxWidth='lg'
                        fullWidth
                        style={{ zIndex: 100 }}
                        onKeyDown={this.submit}
                        PaperProps={{
                            sx: {
                                width: 600,
                                maxHeight: 300
                            }
                        }}
                    >
                        <DialogTitle id="responsive-dialog-title">
                            {"Default Menu"}
                        </DialogTitle>
                        <DialogContent PaperProps={{
                            sx: {
                                width: 600,
                                maxHeight: 300
                            }
                        }}>
                            <DialogContentText>
                                This is a default popup.
                            </DialogContentText>
                        </DialogContent>
                    </Dialog>
                </Backdrop>
                <div onClick={(e) => {
                    this.setState({ open: true })
                    console.log("Clicked")
                    e.stopPropagation();
                    e.preventDefault();
                    e.nativeEvent.stopImmediatePropagation();
                }}>
                    {this.props.children}
                </div>
            </>
        );
    }
}