import Box from "@mui/material/Box";
import axios from 'axios';
import {useEffect, useState} from "react";
import {
    Alert,
    Button,
    Card,
    CardContent, CircularProgress, Dialog, DialogActions, DialogContent, DialogTitle,
    FormControl, Input,
    InputLabel, Paper,
    Select, Table,
    TableBody,
    TableCell, TableContainer,
    TableHead, TableRow
} from "@mui/material";
import Typography from "@mui/material/Typography";
import {ThemeProvider, useTheme} from "@mui/styles";
import MenuItem from "@mui/material/MenuItem";

function Dashboard() {
    const [file, setFile] = useState(null);
    const [postResponseData, setPostResponseData] = useState([]);
    const [showParseWarning, setShowParseWarning] = useState(false);
    const [showServerWarning, setShowServerWarning] = useState(false);
    const [showPopup, setShowPopup] = useState(false);
    const [loadingReport, setLoadingReport] = useState(true);
    const [dropDown, setDropDown] = useState([]);
    const [reportData, setReportData] = useState(null);
    const [selectedOption, setSelectedOption] = useState('');

    const theme = useTheme()

    useEffect(() => {},[reportData])

    useEffect(() => {
        axios({
            method: 'get',
            url: 'http://localhost:3001/xmls',
        })
            .then(response => {
                const responseData = response.data;
                console.log(responseData)
                setDropDown(responseData);
            })
            .catch(error => {
                setShowServerWarning(true)
                console.error('Error:', error);
            });
        // Check if data has changed and show the pop-up
        if (postResponseData.length > 0) {
            setShowPopup(true);
        }
    }, [postResponseData]);

    useEffect(() => {
        if (reportData === null || reportData === undefined) {
            return
        }
        if (reportData.processing) {
            setShowParseWarning(true)
        } else {
            setShowParseWarning(false)
        }
    }, [reportData]);

    const handleFileChange = event => {
        const uploadedFile = event.target.files[0];
        if (uploadedFile) {
            setFile({ name: uploadedFile.name, file: uploadedFile });
            console.log(`Uploaded file: ${uploadedFile.name}`);
        }
    };

    const handleCloseParseWarning = () => {
        setShowParseWarning(false);
    };

    const handleCloseServerWarning = () => {
        setShowServerWarning(false);
    };

    const handleXmlChange = (event) => {
        let option = event.target.value;
        setSelectedOption(option);
        refreshReport(option)
    };

    const refreshReport = (option) => {
        setLoadingReport(true);

        axios({
            method: 'get',
            url: `http://localhost:3001/reports?xml_id=${option}`,
        })
            .then(response => {
                const responseData = response.data;
                console.log(responseData)
                setReportData(responseData);
                setLoadingReport(false);
            })
            .catch(error => {
                setShowServerWarning(true)
                console.error('Error:', error);
            });
    };

    const handleClosePopup = () => {
        setShowPopup(false);
    };

    const handleUpload = () => {
        console.log("posting");
        if (file === undefined || file === null) {
            console.log("No file provided");
            return
        }

        const formData = new FormData();

        formData.append(file.name, file.file);

        axios({
            method: 'post',
            url: 'http://localhost:3001/transactions',
            data: formData,
            headers: {
                'Content-Type': `multipart/form-data; boundary=${formData._boundary}`,
            },
        })
            .then(response => {
                const responseData = response.data;
                setPostResponseData(responseData);
                console.log(postResponseData)
            })
            .catch(error => {
                setShowServerWarning(true)
                console.error('Error:', error);
            });
    };

    const convertToCSV = (data) => {
        let csv = []

        csv.push(`Report for xml: ${data.xml_id}`);
        if (data.processing) {
            csv.push(`\nWarning: Parsing has not completed for XML: ${data.xml_id}. Report is incomplete`);
        }
        if (data.payment_statuses.length) {
            data.payment_statuses = data.payment_statuses.map(pm_status => {
                return { ...pm_status, amount: "$" + pm_status.amount / 100 };
            });
            const ps_title = "\nPayment statuses";
            const ps_head = Object.keys(data.payment_statuses[0]).join(',');
            const ps_body = data.payment_statuses.map(row => Object.values(row).join(',')).join('\n');
            csv.push([ps_title, ps_head, ps_body].join('\n'))
        }

        const pmb_title = "\nFunds distributed per branch";
        const branches = ["Branch,"] + Object.keys(data.payment_map_branch).join(',');
        const spent_branch = ["Spent,"] + Object.values(data.payment_map_branch).map(money => "$" + money/100).join(',');
        csv.push([pmb_title, branches, spent_branch].join('\n'))

        const pma_title = "\nFunds distributed per account";
        const accounts = ["Account,"] + Object.keys(data.payment_map_acc).join(',');
        const spent_acc = ["Spent,"] + Object.values(data.payment_map_acc).map(money => "$" + money/100).join(',');
        csv.push([pma_title, accounts, spent_acc].join('\n'))

        return csv;
    };

    const handleDownload = () => {
        const csv = convertToCSV(reportData);
        const blob = new Blob([csv], { type: 'text/csv' });
        const url = URL.createObjectURL(blob);
        const link = document.createElement('a');
        link.href = url;
        link.download = `report_xml${reportData.xml_id}.csv`;
        link.click();
        URL.revokeObjectURL(url);
    };

    return(
        <ThemeProvider theme={theme}>
            <Box
            sx={{
                borderRadius: 10,
                backgroundColor: theme.palette.secondary.main,
                display: 'flex',
                flexDirection: 'column',
                maxHeight: '75vh',
                overflow: "auto"
            }}>
                <Typography variant="h6">View Reports</Typography>
                <Card>
                    <CardContent>
                        <FormControl sx={{ marginTop: '0.5rem'}}>
                            <InputLabel id="dropdown-label">Select an option</InputLabel>
                            <Select sx={{minWidth: '8rem'}}
                                labelId="dropdown-label"
                                id="dropdown"
                                value={selectedOption}
                                onChange={handleXmlChange}
                            >
                                {dropDown.map((option, index) => (
                                        <MenuItem key={index} value={option.id}>Xml Id:{option.id}</MenuItem>
                                    )
                                )}
                            </Select>
                        </FormControl>
                    </CardContent>
                </Card>
                <Card sx={{flex: 1}}>
                    <CardContent>
                        {showServerWarning && (
                            <Alert onClose={handleCloseServerWarning} severity="error" sx={{ width: '100%' }}>
                                Error communicating with API. Please try again later.
                            </Alert>
                        )}
                        {showParseWarning && (
                            <Alert onClose={handleCloseParseWarning} severity="warning" sx={{ width: '100%' }}>
                                Parsing has not completed for this xml
                                <Button
                                    style={{
                                        display: !loadingReport ? 'block' : 'none',
                                        backgroundColor: theme.palette.secondary.main
                                    }}
                                    onClick={() => refreshReport(selectedOption)}
                                    variant="contained"
                                >
                                    Refresh
                                </Button>
                            </Alert>
                        )}
                        {reportData === undefined || reportData === null ? (
                            <Typography variant="h6">Select an XML from the menu</Typography>
                        ) : loadingReport ? (
                            <CircularProgress />
                        ) : (
                            <>
                                <Box sx={{height:'50vh',  overflow: "auto"}}>
                                    <Typography variant="h6">Funds distributed per branch</Typography>
                                    <TableContainer sx={{ maxHeight: '50%', overflow: "auto" }} component={Paper}>
                                        <Table sx={{ minWidth: 650 }} aria-label="simple table">
                                            <TableHead>
                                                <TableRow>
                                                    <TableCell><b>Branch</b></TableCell>
                                                    <TableCell align="right"><b>Spent</b></TableCell>
                                                </TableRow>
                                            </TableHead>
                                            <TableBody>
                                                {Object.keys(reportData.payment_map_branch).map((branch, index) => (
                                                    <TableRow key={index} sx={{ '&:last-child td, &:last-child th': { border: 0 } }}>
                                                        <TableCell component="th" scope="row">
                                                            {branch}
                                                        </TableCell>
                                                        <TableCell align="right">${(reportData.payment_map_branch[branch] / 100).toFixed(2)}</TableCell>
                                                    </TableRow>
                                                ))}
                                            </TableBody>
                                        </Table>
                                    </TableContainer>
                                    <Typography variant="h6" sx={{ marginTop: '1rem'}}>Funds distributed per account </Typography>
                                    <TableContainer sx={{ maxHeight: '50%', overflow: "auto" }} component={Paper}>
                                        <Table sx={{ minWidth: 650 }} aria-label="simple table">
                                            <TableHead>
                                                <TableRow>
                                                    <TableCell><b>Payor Account</b></TableCell>
                                                    <TableCell align="right"><b>Spent</b></TableCell>
                                                </TableRow>
                                            </TableHead>
                                            <TableBody>
                                                {Object.keys(reportData.payment_map_acc).map((account, index) => (
                                                    <TableRow key={index} sx={{ '&:last-child td, &:last-child th': { border: 0 } }}>
                                                        <TableCell component="th" scope="row">
                                                            {account}
                                                        </TableCell>
                                                        <TableCell align="right">${(reportData.payment_map_acc[account] / 100).toFixed(2)}</TableCell>
                                                    </TableRow>
                                                ))}
                                            </TableBody>
                                        </Table>
                                    </TableContainer>
                                    <Typography variant="h6" sx={{ marginTop: '1rem'}}>Payment Statues</Typography>
                                    <TableContainer sx={{ maxHeight: '50%', overflow: "auto" }} component={Paper}>
                                        <Table sx={{ minWidth: 650 }} aria-label="simple table">
                                            <TableHead>
                                                <TableRow>
                                                    <TableCell><b>Id</b></TableCell>
                                                    <TableCell align="right"><b>Status</b></TableCell>
                                                    <TableCell align="right"><b>Source</b></TableCell>
                                                    <TableCell align="right"><b>Destination</b></TableCell>
                                                    <TableCell align="right"><b>Estimated Completion</b></TableCell>
                                                    <TableCell align="right"><b>Amount</b></TableCell>
                                                </TableRow>
                                            </TableHead>
                                            <TableBody>
                                                {reportData.payment_statuses.map((status, index) => (
                                                    <TableRow key={index} sx={{ '&:last-child td, &:last-child th': { border: 0 } }}>
                                                        <TableCell align="right">{status.id}</TableCell>
                                                        <TableCell align="right">{status.status}</TableCell>
                                                        <TableCell align="right">{status.source}</TableCell>
                                                        <TableCell align="right">{status.destination}</TableCell>
                                                        <TableCell align="right">{status.estimated_completion_date}</TableCell>
                                                        <TableCell align="right">${(status.amount / 100).toFixed(2)}</TableCell>
                                                    </TableRow>
                                                ))}
                                            </TableBody>
                                        </Table>
                                    </TableContainer>
                                </Box>
                                <Box>
                                    <Button variant="contained" onClick={handleDownload}
                                            sx={{backgroundColor: theme.palette.secondary.main}}>
                                        Download Report
                                    </Button>
                                </Box>
                            </>
                        )}
                    </CardContent>
                </Card>
                <Dialog open={showPopup} onClose={handleClosePopup}>
                    <DialogTitle>Data Uploaded</DialogTitle>
                    <DialogContent>
                        {postResponseData.length > 0 && (
                            <div>
                                <p>XML Uploaded Successfully</p>
                                <p>id: {postResponseData[0].id}</p>
                                <p>filename: {postResponseData[0].filename}</p>
                                <p>status: {postResponseData[0].status}</p>
                                <p>started_at: {postResponseData[0].started_at}</p>
                            </div>
                        )}
                    </DialogContent>
                    <DialogActions>
                        <Button onClick={handleClosePopup}>Close</Button>
                    </DialogActions>
                </Dialog>
                <Card>
                    <CardContent>
                        <Typography variant="h6">Upload File</Typography>
                        <Input type="file" onInput={handleFileChange}/>
                        <Button variant="contained" onClick={handleUpload}
                                sx={{backgroundColor: theme.palette.secondary.main}}>Upload</Button>
                    </CardContent>
                </Card>
            </Box>
        </ThemeProvider>
    )
}

export default Dashboard
