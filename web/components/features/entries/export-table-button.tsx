"use client";

import { useState } from "react";
import { Image as ImageIcon } from "lucide-react";
import html2canvas from "html2canvas";
import { Button } from "@/components/ui/button";
import { toast } from "sonner";
import { useTranslations } from "next-intl";

interface ExportTableButtonProps {
  readonly tableRef: React.RefObject<HTMLElement>;
  readonly filename?: string;
  readonly budgetName?: string;
  readonly dateRange?: string;
}

export function ExportTableButton({
  tableRef,
  filename = "entries-table",
  budgetName = "Budget",
  dateRange,
}: ExportTableButtonProps) {
  const t = useTranslations();
  const [isExporting, setIsExporting] = useState(false);

  const createStyledExport = async () => {
    if (!tableRef.current) return null;

    // Load logo
    const logoImg = new Image();
    logoImg.crossOrigin = "anonymous";
    logoImg.src = "/philand.png";

    await new Promise<void>((resolve, reject) => {
      logoImg.onload = () => resolve();
      logoImg.onerror = () => reject(new Error("Logo load failed"));
    });

    // OUTER WRAPPER
    const wrapper = document.createElement("div");
    wrapper.style.cssText = `
      position: absolute;
      left: -9999px;
      top: 0;
      padding: 56px 32px 80px 32px;
      background: radial-gradient(circle at top left,#a855f7 0,#4f46e5 35%,#0f172a 100%);
      font-family: system-ui,-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,sans-serif;
      line-height: 1.6;
      min-width: 1400px;
      display: flex;
      justify-content: center;
      align-items: center;
      box-sizing: border-box;
    `;

    // FRAME
    const frame = document.createElement("div");
    frame.style.cssText = `
      width: 1300px;
      max-width: 1300px;
      background: #f9fafb;
      border-radius: 32px;
      box-shadow: 0 24px 60px rgba(15,23,42,0.45);
      overflow: visible;
      display: flex;
      flex-direction: column;
    `;

    // HEADER
    const header = document.createElement("div");
    header.style.cssText = `
      padding: 32px 40px 24px 40px;
      background: linear-gradient(to bottom,#f9fafb,#eef2ff);
      border-bottom: 1px solid #e5e7eb;
    `;
    header.innerHTML = `
      <div style="display:flex;justify-content:space-between;align-items:flex-start;gap:20px;">
        <div style="flex:1;min-width:0;">
          <h1 style="
            margin:4px 0 6px 0;
            padding:4px 0;
            font-size:26px;             /* smaller title */
            font-weight:800;
            color:#020617;
            letter-spacing:-0.04em;
            white-space:nowrap;
            overflow:hidden;
            text-overflow:ellipsis;
            line-height:1.4;
          ">
            ${budgetName}
          </h1>
          ${
            dateRange
              ? `<p style="
                    margin:0;
                    padding:2px 0 4px 0;
                    font-size:13px;
                    color:#6b7280;
                    font-weight:500;
                    line-height:1.5;
                  ">${dateRange}</p>`
              : ""
          }
        </div>
        <div style="
          padding:12px 18px;
          border-radius:999px;
          background:#0f172a;
          display:flex;
          flex-direction:column;
          align-items:flex-end;
          gap:3px;
        ">
          <span style="
            font-size:11px;
            text-transform:uppercase;
            letter-spacing:0.12em;
            color:#9ca3af;
            font-weight:600;
            line-height:1.3;
          ">Exported</span>
          <span style="
            font-size:13px;
            font-weight:600;
            color:#e5e7eb;
            line-height:1.4;
          ">
            ${new Date().toLocaleDateString("en-US", {
              year: "numeric",
              month: "short",
              day: "numeric",
            })}
          </span>
        </div>
      </div>
    `;

    // CONTENT
    const content = document.createElement("div");
    content.style.cssText = `
      padding: 28px 40px 44px 40px;
      background:#f9fafb;
    `;

    const appShell = document.createElement("div");
    appShell.style.cssText = `
      border-radius: 24px;
      background:#020617;
      padding: 26px 24px 32px 24px;
      box-shadow: 0 16px 40px rgba(15,23,42,0.7);
      border:1px solid rgba(148,163,184,0.25);
    `;

    const appHeader = document.createElement("div");
    appHeader.style.cssText = `
      display:flex;
      justify-content:space-between;
      align-items:center;
      margin-bottom:16px;
    `;
    appHeader.innerHTML = `
      <div style="
        font-size:14px;                /* smaller label */
        font-weight:600;
        color:#e5e7eb;
        letter-spacing:0.08em;
        text-transform:uppercase;
        line-height:1.3;
      ">
        Transactions
      </div>
      <div style="
        display:flex;
        gap:16px;
        font-size:12px;
        font-weight:500;
        line-height:1.4;
      " id="philand-export-summary-slot"></div>
    `;

    const clonedTableRoot = tableRef.current.cloneNode(true) as HTMLElement;
    clonedTableRoot.querySelectorAll("button").forEach((btn) => btn.remove());
    clonedTableRoot
      .querySelectorAll('[role="menu"],[data-radix-popper-content-wrapper]')
      .forEach((d) => d.remove());

    const summaryContainer =
      clonedTableRoot.querySelector("[data-export-summary]") ||
      clonedTableRoot.querySelector('[class*="CardHeader"] div div[class*="flex gap-4"]') ||
      clonedTableRoot.querySelector('[class*="CardHeader"] div div:last-child');
    
    console.log('Summary container found:', !!summaryContainer);
    if (summaryContainer) {
      console.log('Summary HTML:', summaryContainer.innerHTML);
      const summarySlot = appHeader.querySelector(
        "#philand-export-summary-slot"
      ) as HTMLElement | null;
      if (summarySlot) {
        summarySlot.style.cssText += `
          color:#e5e7eb;
          display:flex;
          gap:16px;
        `;
        summarySlot.innerHTML = summaryContainer.innerHTML;
        
        // Style the summary stats with proper colors
        const summaryItems = summarySlot.querySelectorAll('div');
        summaryItems.forEach((item, index) => {
          const spans = item.querySelectorAll('span');
          if (spans.length >= 2) {
            // First span is the label
            (spans[0] as HTMLElement).style.cssText = `
              color: #9ca3af;
              font-size: 12px;
            `;
            // Second span is the value
            const valueSpan = spans[1] as HTMLElement;
            valueSpan.style.cssText = `
              font-weight: 600;
              font-size: 12px;
            `;
            
            // Apply colors based on type
            if (index === 0) { // Income
              valueSpan.style.color = '#10b981';
            } else if (index === 1) { // Expense  
              valueSpan.style.color = '#ef4444';
            } else if (index === 2) { // Net
              const text = valueSpan.textContent || '';
              const isNegative = text.includes('-');
              valueSpan.style.color = isNegative ? '#ef4444' : '#10b981';
            }
          }
        });
      }
    }

    const table = clonedTableRoot.querySelector("table") as HTMLElement | null;

    if (table) {
      const tableWrapper = document.createElement("div");
      tableWrapper.style.cssText = `
        margin-top:12px;
        border-radius:18px;
        background:#020617;
        border:1px solid rgba(148,163,184,0.4);
        overflow:visible;          /* IMPORTANT: don't clip diacritics */
      `;

      table.style.cssText = `
        width:100%;
        border-collapse:collapse;
        font-size:12px;            /* smaller body text */
      `;

      const thead = table.querySelector("thead") as HTMLElement | null;
      if (thead) {
        thead.style.cssText = `
          background:#020617;
          border-bottom:1px solid rgba(148,163,184,0.6);
        `;
      }

      const headers = table.querySelectorAll("th");
      headers.forEach((th) => {
        const el = th as HTMLElement;
        const text = (el.textContent || "").toLowerCase();

        el.style.cssText = `
          padding:10px 12px;
          font-size:9px;
          font-weight:600;
          text-transform:uppercase;
          letter-spacing:0.06em;
          color:#9ca3af;
          text-align:left;
          white-space:nowrap;
          line-height:1.4;
        `;
        if (text.includes("amount")) el.style.textAlign = "right";
      });

      const tbody = table.querySelector("tbody") as HTMLElement | null;
      if (tbody) {
        const rows = tbody.querySelectorAll("tr");
        rows.forEach((tr) => {
          const rowEl = tr as HTMLElement;
          rowEl.style.cssText = `
            background:#020617;
          `;

          const cells = rowEl.querySelectorAll("td");
          cells.forEach((td, cellIndex) => {
            const cellEl = td as HTMLElement;
            const text = (cellEl.textContent || "").trim();

            cellEl.style.cssText = `
              padding:10px 12px;
              font-size:11px;
              color:#e5e7eb;
              vertical-align:middle;
              border-bottom:1px solid rgba(15,23,42,0.9);
              line-height:1.6;
              max-width:200px;
              overflow:visible;
              word-wrap:break-word;
              ${cellIndex === cells.length - 1 ? "text-align:right;" : ""}
            `;

            if (text.match(/[$€£¥₫]|^-?\d[\d,.]*$/)) {
              cellEl.style.textAlign = "right";
              cellEl.style.fontWeight = "600";
              cellEl.style.whiteSpace = "nowrap";
            }

            // Style category cells with icon + text
            const categoryDiv = cellEl.querySelector('div[class*="flex"]') as HTMLElement | null;
            if (categoryDiv) {
              categoryDiv.style.cssText = `
                display:flex;
                align-items:center;
                gap:8px;
                max-width:100%;
              `;
              
              // Style icon container
              const iconContainer = categoryDiv.querySelector('div[class*="rounded"]') as HTMLElement | null;
              if (iconContainer) {
                iconContainer.style.cssText = `
                  flex-shrink:0;
                  width:24px;
                  height:24px;
                  display:flex;
                  align-items:center;
                  justify-content:center;
                  border-radius:6px;
                `;
              }
              
              // Style category name
              const nameSpan = categoryDiv.querySelector('span') as HTMLElement | null;
              if (nameSpan) {
                nameSpan.style.cssText = `
                  font-size:11px;
                  font-weight:500;
                  line-height:1.4;
                  overflow:visible;
                  word-wrap:break-word;
                `;
              }
            }

            const badge = cellEl.querySelector(
              'span[class*="inline-flex"],span[class*="badge"]'
            ) as HTMLElement | null;
            if (badge) {
              const label = (badge.textContent || "").toLowerCase();
              const isIncome = label.includes("income");
              badge.style.cssText = `
                display:inline-flex;
                align-items:center;
                justify-content:center;
                padding:3px 10px;
                border-radius:999px;
                font-size:9px;
                font-weight:600;
                letter-spacing:0.04em;
                text-transform:uppercase;
                line-height:1.3;
                white-space:nowrap;
                background:${
                  isIncome ? "rgba(22,163,74,0.18)" : "rgba(220,38,38,0.18)"
                };
                color:${isIncome ? "#bbf7d0" : "#fecaca"};
                border:1px solid ${
                  isIncome ? "rgba(34,197,94,0.5)" : "rgba(248,113,113,0.5)"
                };
              `;
            }
          });
        });
      }

      tableWrapper.appendChild(table);
      appShell.appendChild(appHeader);
      appShell.appendChild(tableWrapper);
    } else {
      appShell.appendChild(appHeader);
      appShell.appendChild(clonedTableRoot);
    }

    content.appendChild(appShell);

    // FOOTER
    const footer = document.createElement("div");
    footer.style.cssText = `
      padding: 24px 40px 30px 40px;
      background:#f9fafb;
      border-top:1px solid #e5e7eb;
      display:flex;
      justify-content:space-between;
      align-items:center;
      gap:18px;
    `;

    const logoCanvas = document.createElement("canvas");
    logoCanvas.width = 48;
    logoCanvas.height = 48;
    const ctx = logoCanvas.getContext("2d");
    if (ctx) {
      ctx.clearRect(0, 0, 48, 48);
      ctx.drawImage(logoImg, 0, 0, 48, 48);
    }
    const logoDataUrl = logoCanvas.toDataURL();

    footer.innerHTML = `
      <div style="display:flex;align-items:center;gap:16px;">
        <img src="${logoDataUrl}" alt="Philand" style="
          width:40px;
          height:40px;
          border-radius:12px;
          object-fit:cover;
          box-shadow:0 6px 12px rgba(15,23,42,0.25);
        " />
        <div>
          <div style="
            font-size:14px;
            font-weight:800;
            color:#020617;
            letter-spacing:-0.04em;
            margin-bottom:2px;
            line-height:1.4;
          ">
            Philand
          </div>
          <div style="
            font-size:11px;
            color:#6b7280;
            font-weight:500;
            line-height:1.5;
          ">
            Budget Tracking &amp; Financial Management
          </div>
        </div>
      </div>
      <div style="text-align:right;">
        <div style="
          font-size:12px;
          font-weight:600;
          color:#4f46e5;
          margin-bottom:2px;
          line-height:1.4;
        ">
          phila.cloud
        </div>
        <div style="
          font-size:10px;
          color:#9ca3af;
          line-height:1.4;
        ">
          Made with ❤️
        </div>
      </div>
    `;

    frame.appendChild(header);
    frame.appendChild(content);
    frame.appendChild(footer);
    wrapper.appendChild(frame);
    document.body.appendChild(wrapper);

    try {
      const canvas = await html2canvas(frame, {
        backgroundColor: null,
        scale: 2,
        logging: false,
        useCORS: true,
        allowTaint: true,
      });

      wrapper.remove();
      return canvas;
    } catch (err) {
      wrapper.remove();
      throw err;
    }
  };

  const handleExport = async () => {
    if (!tableRef.current) {
      toast.error(t("common.error"), { description: t("entry.exportFailed") });
      return;
    }

    setIsExporting(true);
    try {
      await new Promise((resolve) => setTimeout(resolve, 80));

      const canvas = await createStyledExport();
      if (!canvas) throw new Error("Failed to create canvas");

      canvas.toBlob((blob) => {
        if (!blob) {
          toast.error(t("common.error"), {
            description: "Failed to create image",
          });
          return;
        }
        const url = URL.createObjectURL(blob);
        const link = document.createElement("a");
        const timestamp = new Date().toISOString().split("T")[0];
        link.download = `${filename}-${timestamp}.png`;
        link.href = url;
        link.click();
        URL.revokeObjectURL(url);

        toast.success(t("entry.exportSuccess"), {
          description: t("entry.exportSuccessDesc"),
        });
      }, "image/png");
    } catch (error) {
      console.error("Export error:", error);
      toast.error(t("common.error"), {
        description:
          error instanceof Error ? error.message : t("entry.exportFailed"),
      });
    } finally {
      setIsExporting(false);
    }
  };

  return (
    <Button
      variant="outline"
      onClick={handleExport}
      disabled={isExporting}
      className="gap-2 h-12 sm:h-10 text-base sm:text-sm font-medium w-full sm:w-auto"
    >
      {isExporting ? (
        <>
          <div className="h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent" />
          {t("entry.exporting")}
        </>
      ) : (
        <>
          <ImageIcon className="h-5 w-5 sm:h-4 sm:w-4" />
          {t("entry.exportAsImage")}
        </>
      )}
    </Button>
  );
}