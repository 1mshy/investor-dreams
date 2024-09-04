import { Backdrop, Button, DialogContent, DialogContentText } from "@mui/material";
import React, { Component } from "react";


import DialogTitle from '@mui/material/DialogTitle';
import Dialog from '@mui/material/Dialog';
import { Transition } from "@/app/funcs/themes";
import { invoke } from "@tauri-apps/api/core";
import { upload_json } from "@/app/funcs/tools";

class TableDownloadPopup extends Component {
    constructor(props) {
        super(props);
        this.state = {
            char_name: props.char_name, open: !!props.open,
        };
        this.handleClose = this.handleClose.bind(this);
    }

    handleClose(event, reason) {
        if (reason && reason === "backdropClick") {
            this.setState({ open: false });
        }
    }


    render() {
        const { open } = this.state;
        const { downloadable_stores } = this.props;
        console.log(downloadable_stores[0])
        return (
            <>
                <Backdrop open={open} invisible={true}>
                    <Dialog
                        open={open}
                        onClose={this.handleClose}
                        aria-labelledby="responsive-dialog-title"
                        TransitionComponent={Transition}
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
                            {"Table Downloads"}
                        </DialogTitle>
                        <DialogContent PaperProps={{
                            sx: {
                                width: 600,
                                maxHeight: 300
                            }
                        }}>
                            Click on any of the tables to download them:

                            {downloadable_stores.map((table, index) => {
                                return <div>
                                    <Button key={index} onClick={async () => {
                                        console.log(table)
                                        const keys = await table.keys();
                                        const data = await Promise.all(keys.map(async key => {
                                            const value = await table.getItem(key);
                                            return { key, value };
                                        }));
                                        const json_table = {};
                                        data.forEach(({ key, value }) => {
                                            json_table[key] = value;
                                        });
                                        const filename = `${table._dbInfo.db.name}.json`;
                                        upload_json(json_table, filename);

                                    }}>
                                        {`${table._dbInfo.db.name}`}
                                    </Button>
                                </div>
                            })}
                        </DialogContent>
                    </Dialog>
                </Backdrop>
                <div onClick={() => {
                    this.setState({ open: true })
                }}>
                    {this.props.children}
                </div>
            </>
        );
    }
}

export default TableDownloadPopup;