import './shared.css';

interface ConfirmModalProps {
    message: string;
    onConfirm: () => void;
    onCancel: () => void;
}

function ConfirmModal({ message, onConfirm, onCancel }: ConfirmModalProps) {
    return (
        <>
            <div className="modal-container">
                <div className="modal-card">
                    <p className="modal-message">{message}</p>
                    <div className="modal-btn-row">
                        <button
                            className="modal-confirm-btn"
                            onClick={() => onConfirm()}
                        >
                            Confirm
                        </button>
                        <button
                            className="modal-cancel-btn"
                            onClick={() => onCancel()}
                        >
                            Cancel
                        </button>
                    </div>
                </div>
            </div>
        </>
    );
}

export default ConfirmModal;