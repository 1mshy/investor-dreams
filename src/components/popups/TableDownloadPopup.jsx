import { Backdrop, Button, DialogContent } from "@mui/material";
import { Component } from "react";
import { Transition } from "@/app/funcs/themes";
import { write_chunks } from "@/app/funcs/tools";
import Dialog from '@mui/material/Dialog';
import DialogTitle from '@mui/material/DialogTitle';

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
                    >
                        <DialogTitle id="responsive-dialog-title">
                            {"Table Downloads"}
                        </DialogTitle>
                        <DialogContent>
                            Click on any of the tables to download them:

                            {downloadable_stores.map((table, index) => {
                                if(!table._config.name) return <div key={`${index}_${this.state.open}`}/>;
                                return <div key={`${index}_${this.state.open}`}>
                                    <Button key={index} onClick={async () => {
                                        const keys = await table.keys();
                                        const data = await Promise.all(keys.map(async key => {
                                            const value = await table.getItem(key);
                                            return { key, value };
                                        }));
                                        const json_table = {};
                                        data.forEach(({ key, value }) => {
                                            json_table[key] = value;
                                        });
                                        // const filename = `${table._dbInfo.db.name}.json`;
                                        const folder = `${table._config.name}`;
                                        write_chunks(json_table, folder);
                                    }}>
                                        {`${table._config.name}`}
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