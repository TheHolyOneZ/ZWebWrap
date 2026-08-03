import { Modal } from "./Modal";
import { useT } from "../i18n/useI18n";

interface Props {
  title: string;
  body: string;
  confirmLabel: string;
  onConfirm: () => void;
  onCancel: () => void;
}

export function ConfirmDialog({
  title,
  body,
  confirmLabel,
  onConfirm,
  onCancel,
}: Props) {
  const t = useT();
  return (
    <Modal
      title={title}
      onClose={onCancel}
      footer={
        <>
          <div className="spacer" />
          <button className="btn" onClick={onCancel}>
            {t("ui.cancel")}
          </button>
          <button className="btn btn--danger" onClick={onConfirm} autoFocus>
            {confirmLabel}
          </button>
        </>
      }
    >
      <p className="dialog__body">{body}</p>
    </Modal>
  );
}
