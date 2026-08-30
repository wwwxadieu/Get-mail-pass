export type Toast = { id: number; text: string; kind: "ok" | "err" };

export default function Toasts({ items }: { items: Toast[] }) {
  if (items.length === 0) return null;
  return (
    <div className="toasts">
      {items.map((t) => (
        <div key={t.id} className={t.kind === "err" ? "toast err" : "toast"}>
          {t.text}
        </div>
      ))}
    </div>
  );
}
