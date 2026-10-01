import "./layout.css";
import { AuthContext } from "../context/AuthContext";
import { useContext } from "react";
import { useNavigate } from "react-router-dom";

function TopBar() {
    const { isDesktop } = useContext(AuthContext);
    const navigate = useNavigate();

    if (!isDesktop) return null;

    return (
        <>
            <div className="topbar">
                <div className="topbar-nav">
                    <button
                        className="topbar-nav-btn"
                        onClick={() => navigate(-1)}
                    >
                        ←
                    </button>
                    <button
                        className="topbar-nav-btn"
                        onClick={() => navigate(1)}
                    >
                        →
                    </button>
                </div>

                <div className="topbar-title">
                    <svg
                        className="topbar-logo"
                        width="32"
                        height="32"
                        viewBox="0 0 32 32"
                    >
                        <circle cx="16" cy="16" r="16" fill="var(--accent)" />
                        <text
                            x="16"
                            y="21"
                            textAnchor="middle"
                            fill="var(--text-on-dark)"
                            fontSize="16"
                            fontWeight="bold"
                        >
                            C
                        </text>
                    </svg>
                    <h1 className="topbar-app-name">ChatApp</h1>
                </div>

                <div className="topbar-spacer" />
            </div>
        </>
    );
}

export default TopBar;