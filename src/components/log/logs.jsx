import moment from "moment"
import { useEffect, useState } from "react"
// import { useDispatch, useSelector } from "react-redux"
import TitleCard from "../TitleCard"
// import { showNotification } from '../../common/headerSlice'



const BILLS = [
    { invoiceNo: "#4567", amount: "-", description: "Z Office Api", status: "Processing", generatedOn: moment(new Date()).add(-30 * 1, 'days').format("DD MMM YYYY"), paidOn: "-" },

    { invoiceNo: "#4523", amount: "-", description: "Fred Pos Api", status: "Processing", generatedOn: moment(new Date()).add(-30 * 2, 'days').format("DD MMM YYYY"), paidOn: "-" },

    { invoiceNo: "#4453", amount: "10 minutes", description: "Updates", status: "Completed", generatedOn: moment(new Date()).add(-30 * 3, 'days').format("DD MMM YYYY"), paidOn: "10 minutes" },

    { invoiceNo: "#4359", amount: "20 minutes", description: "Z Office Api", status: "Failed", generatedOn: moment(new Date()).add(-30 * 4, 'days').format("DD MMM YYYY"), paidOn: "15 minutes" },

    // { invoiceNo: "#3359", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 5, 'days').format("DD MMM YYYY"), paidOn: "5 minutes" },

    // { invoiceNo: "#3367", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 6, 'days').format("DD MMM YYYY"), paidOn: "10 minutes" },

    // { invoiceNo: "#3359", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 7, 'days').format("DD MMM YYYY"), paidOn: "7 minutes" },

    // { invoiceNo: "#2359", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 8, 'days').format("DD MMM YYYY"), paidOn: "-" },
    // { invoiceNo: "#2359", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 8, 'days').format("DD MMM YYYY"), paidOn: "-" },
    // { invoiceNo: "#2359", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 8, 'days').format("DD MMM YYYY"), paidOn: "-" },
    // { invoiceNo: "#2359", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 8, 'days').format("DD MMM YYYY"), paidOn: "10 minutes" },
    // { invoiceNo: "#2359", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 8, 'days').format("DD MMM YYYY"), paidOn: "10 minutes" },
    // { invoiceNo: "#2359", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 8, 'days').format("DD MMM YYYY"), paidOn: "10 minutes" },


    // { invoiceNo: "#2359", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 8, 'days').format("DD MMM YYYY"), paidOn: moment(new Date()).add(-24 * 7, 'days').format("DD MMM YYYY") },
    // { invoiceNo: "#2359", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 8, 'days').format("DD MMM YYYY"), paidOn: moment(new Date()).add(-24 * 7, 'days').format("DD MMM YYYY") },
    // { invoiceNo: "#2359", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 8, 'days').format("DD MMM YYYY"), paidOn: moment(new Date()).add(-24 * 7, 'days').format("DD MMM YYYY") },
    // { invoiceNo: "#2359", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 8, 'days').format("DD MMM YYYY"), paidOn: moment(new Date()).add(-24 * 7, 'days').format("DD MMM YYYY") },
    // { invoiceNo: "#2359", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 8, 'days').format("DD MMM YYYY"), paidOn: moment(new Date()).add(-24 * 7, 'days').format("DD MMM YYYY") },
    // { invoiceNo: "#2359", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 8, 'days').format("DD MMM YYYY"), paidOn: moment(new Date()).add(-24 * 7, 'days').format("DD MMM YYYY") },
    // { invoiceNo: "#2359", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 8, 'days').format("DD MMM YYYY"), paidOn: moment(new Date()).add(-24 * 7, 'days').format("DD MMM YYYY") },
    // { invoiceNo: "#2359", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 8, 'days').format("DD MMM YYYY"), paidOn: moment(new Date()).add(-24 * 7, 'days').format("DD MMM YYYY") },
    // { invoiceNo: "#2359", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 8, 'days').format("DD MMM YYYY"), paidOn: moment(new Date()).add(-24 * 7, 'days').format("DD MMM YYYY") },
    // { invoiceNo: "#2359", amount: "28,927", description: "Product usages", status: "Paid", generatedOn: moment(new Date()).add(-30 * 8, 'days').format("DD MMM YYYY"), paidOn: moment(new Date()).add(-24 * 7, 'days').format("DD MMM YYYY") },


]

function Logs({ config }) {

    const [bills, setBills] = useState(BILLS)

    const getPaymentStatus = (status) => {
        if (status === "Completed") return <div className="badge badge-success">{status}</div>
        if (status === "Processing") return <div className="badge badge-primary">{status}</div>
        if (status === "Failed") return <div className="badge badge-error">{status}</div>
        else return <div className="badge badge-ghost">{status}</div>
    }

    const callEndpoint = (name) => {
        invoke("fetch_data").then((response) => setMessage(response))
    };

    const callCloudServer = () => {
        invoke("send_data").then((response) => setMessage(response))
    };

    return (
        <div className="w-full h-full overflow-y-auto">

            <TitleCard title="Logs" topMargin="mt-6">

                {/* Invoice list in table format loaded constant */}
                <div className="overflow-x-auto w-full">
                    <table className="table w-full">
                        <thead>
                            <tr>
                                <th>Id</th>
                                <th>Create at</th>
                                <th>Description</th>
                                <th>Time</th>
                                <th>Status</th>
                                {/* <th>Completed on</th> */}
                            </tr>
                        </thead>
                        <tbody>
                            {
                                bills.map((l, k) => {
                                    return (
                                        <tr key={k}>
                                            <td>{l.invoiceNo}</td>
                                            <td>{l.generatedOn}</td>
                                            <td>{l.description}</td>
                                            <td>{l.amount}</td>
                                            <td>{getPaymentStatus(l.status)}</td>
                                            {/* <td>{l.paidOn}</td> */}
                                        </tr>
                                    )
                                })
                            }
                        </tbody>
                    </table>
                </div>
            </TitleCard>
            <div className="p-4">
            <div className="grid grid-cols-4 gap-4">
                {config.api_config.map((l, k) => (
                    <button
                        key={k}
                        onClick={() => callEndpoint(l.name)}
                        // className="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 transition duration-300 ease-in-out"
                          className="btn-primary btn py-2 px-4 rounded"
                    >
                        Call {l.name} Endpoint
                    </button>
                ))}
            </div>
            </div>
           
        </div>
    )
}


export default Logs
// this is test