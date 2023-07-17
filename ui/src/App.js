import './App.css';
import ResponsiveAppBar from "./components/AppBar/AppBar";
import HomePage from "./components/HomePage/HomePage";
import Dashboard from "./components/Profile/Profile";
import {createTheme} from "@mui/material";
import {ThemeProvider} from "@mui/styles";
import {useState} from "react";

const theme = createTheme({
  palette: {
    primary: {
      main: '#DD5600', // Dunkin Orange
    },
    secondary: {
        main: '#FFC0CB', // Dunkin Pink
    },
  },
});

function App() {
    const [overlay, setOverlay] = useState(
            <Dashboard />
    );
    return (
        <ThemeProvider theme={theme}>
            <div className="App" style={{ overflow: 'hidden', height: '100vh' }}>
                <ResponsiveAppBar SetOverlay={setOverlay}/>
                <HomePage Overlay={overlay}/>
            </div>
        </ThemeProvider>
    );
}

export default App;
