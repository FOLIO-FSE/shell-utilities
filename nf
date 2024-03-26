infile="${1}"

cp "${infile}" tmp_outfile

firstline="$(head -1 ${infile})"

echo "${firstline}" | grep -qP "\t" && filetype="tab"
echo "${firstline}" | grep -q "|" && filetype="pipe"
echo "${firstline}" | fgrep -q "^" && filetype="carrot"
echo "${firstline}" | grep -q "," && filetype="comma"

if [[ ${filetype} == "pipe" ]];then
	cat "${infile}" |tr '|' '\t' > tmp_outfile
fi
if [[ ${filetype} == "carrot" ]];then
	cat "${infile}" |tr '^' '\t' > tmp_outfile
fi
if [[ ${filetype} == "comma" ]];then
		csvformat -T "${infile}" > tmp_outfile
fi

numrecs=$(wc -l tmp_outfile |cut -d " " -f1)
numfields=$(awk 'BEGIN{FS="\t"}{print NF;exit}' tmp_outfile)
num_labeled_fields=$(head -1 tmp_outfile |tr '\t' '\n' |grep -i [a-z0-9])

echo
echo "$numfields fields $numrecs lines were detected."
head -1 tmp_outfile |tr '\t' '\n' |grep -in [a-z0-9]
echo 
echo "Field number distribution is as follows in descending order of frequency"
echo
sleep 2

num_labeled_fields=$(head -1 tmp_outfile |tr '\t' '\n' |grep -i [a-z0-9] |wc -l)
sleep 2

if [[ $numfields != $num_labeled_fields ]];then
	echo
	echo "$numfields fields detected but $num_labeled_fields fields with labels were detected."
	echo
	echo "WARNING: DISPARITY BETWEEN NUMBER OF FIELDS AND NUMBER OF LABELS USUALLY INDICATES DATA PROBLEMS -- INSPECT FILE."
	echo
fi

awk 'BEGIN{FS="\t"}{print NF}' tmp_outfile |sort |uniq -c |sort -k1nr 
rm tmp_outfile
