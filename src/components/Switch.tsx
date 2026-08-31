type Props = {
  label: string;
  checked: boolean;
  onChange: (v: boolean) => void;
  disabled?: boolean;
};

export default function Switch({ label, checked, onChange, disabled }: Props) {
  return (
    <div className="row">
      <label>{label}</label>
      <span className="switch">
        <input
          type="checkbox"
          checked={checked}
          disabled={disabled}
          onChange={(e) => onChange(e.target.checked)}
        />
        <span className="slider" />
      </span>
    </div>
  );
}
