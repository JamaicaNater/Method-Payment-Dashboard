import Typography from "@mui/material/Typography";
import Box from "@mui/material/Box";
import {ThemeProvider, useTheme} from "@mui/styles";
import {
    Card,
    CardContent, Divider,
    Paper,
    Table,
    TableBody,
    TableCell,
    TableContainer,
    TableRow
} from "@mui/material";
import Avatar from "@mui/material/Avatar";
import IconButton from "@mui/material/IconButton";

function createData(value, attribute) {
    return { value, attribute };
}

const rows = [
    createData('Name', 'John Dunkin'),
    createData('Title', 'CEO'),
    createData('Department', 'Executive Committee'),
    createData('Salary', '$1,376,790.10'),
    createData('Direct Reports', '1,395'),
];

function Profile() {
    const theme = useTheme()

    return (
        <ThemeProvider theme={theme}>
            <div>
                <Box
                    sx={{
                        width: 'fit-content',
                        height: 'fit-content',
                        borderRadius: 10,
                        backgroundColor: theme.palette.secondary.main,
                    }}>
                    <Typography variant="h4">Profile</Typography>
                    <Divider></Divider>
                    <Card>
                        <CardContent>
                            <Box display="flex" alignItems="center">
                                <IconButton sx={{ p: 10 }}>
                                    <Avatar
                                        alt="John Dunkin"
                                        src="https://headshots-inc.com/wp-content/uploads/2021/04/Website-Photo-4-1.png"
                                        sx={{ width: 150, height: 150}} />
                                </IconButton>
                                <div>
                                    <Typography variant="h6" align="left" fontFamily="Modern Font">John Dunkin</Typography>
                                    <Typography variant="h6" align="left" fontFamily="Modern Font">john.dunkin@dunkinsdonuts.com</Typography>
                                </div>
                            </Box>
                        </CardContent>
                    </Card>
                    <Card>
                        <CardContent>
                            <TableContainer component={Paper}>
                                <Table>
                                    <TableBody>
                                        {rows.map((row) => (
                                            <TableRow
                                                key={row.value}
                                                sx={{ '&:last-child td, &:last-child th': { border: 0 } }}
                                            >
                                                <TableCell align="left">{row.value}</TableCell>
                                                <TableCell align="right">{row.attribute}</TableCell>
                                            </TableRow>
                                        ))}
                                    </TableBody>
                                </Table>
                            </TableContainer>
                        </CardContent>
                    </Card>
                </Box>
            </div>
        </ThemeProvider>
    );
}

export default Profile
