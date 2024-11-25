import { Component } from "solid-js";

interface QuitIconProps {
    width: number;
    height: number;
    color: string;
}

const QuitIcon: Component<QuitIconProps> = (props) => {
    return (
        <svg
            fill={props.color}
            width={`${props.width}px`}
            height={`${props.height}px`}
            viewBox="0 0 64 64"
            version="1.1"
        >
            <path d="M40,14.62c7.91,3.16 13.505,10.896 13.505,19.928c0,11.838 -9.611,21.449 -21.449,21.449c-11.838,0 -21.449,-9.611 -21.449,-21.449c0,-8.99 5.542,-16.695 13.393,-19.883l0,4.332c-5.616,2.918 -9.456,8.789 -9.456,15.551c0,9.665 7.847,17.512 17.512,17.512c9.665,0 17.512,-7.847 17.512,-17.512c0,-6.806 -3.891,-12.711 -9.568,-15.608l0,-4.32Z" />
            <path d="M34.056,9.886c0,-1.104 -0.896,-2 -2,-2c-1.104,0 -2,0.896 -2,2l0,24c0,1.104 0.896,2 2,2c1.104,0 2,-0.896 2,-2l0,-24Z" />
        </svg>
    );
};

export default QuitIcon;
