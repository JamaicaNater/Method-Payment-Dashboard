import { makeStyles } from '@mui/styles';

const useStyles = makeStyles((theme) => ({
    container: {
        position: 'relative',
        width: '100vw',
        height: '100vh',
        overflow: 'hidden',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
    },
    image: {
        width: '100%',
        height: '100%',
        objectFit: 'cover',
    },
    overlay: {
        position: 'absolute',
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        backgroundColor: 'rgba(0, 0, 0, 0.5)', // Example overlay background color
        color: 'white', // Example overlay text color
        fontSize: '2rem', // Example overlay text size
    },
}));

function HomePage({Overlay}) {
    const classes = useStyles();

    return (
        <div className={classes.container}>
            <img
                src="https://hips.hearstapps.com/hmg-prod/images/dunkin-donuts-royalty-free-image-802391746-1532983899.jpg"
                className={classes.image}
                alt="Dunkin Donuts"
            />
            <div className={classes.overlay}>
                {Overlay}
            </div>
        </div>
    );
}

export default HomePage;
